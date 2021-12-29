# Boot and Runtime

## What you will learn
* Minimum requirements to run a binary file on CortexM4
* How to write and use a linker script
* Running a program without standard library
* Automatic compilation and flashing
* Inspect device state with gdb debugger 

## Introduction

From a software engineering perspective the development has usually always the
same start point - no matter what language you use, somewhere must be a main()
function as program entry. By calling the compiled program everything is already 
prepared, the hardware is in a useable state und when entering the main() function
things usually happens as expected. The goal of this chapter is to go a big step back and examine the situation before any platform is booted nor any hardware initialization is done - simply, there is only a piece of hardware and the core features of the programming language Rust. As stated in the chapter before, this is only possible with cross compiling, our host system is the platform for developing and compiling and last but not least for flashing the compiled binary file to the board. Lets dive in!

## Hardware prerequisites

### CPU Bootup
Due to the lack of any abstraction layer it is necessary to deal with the most low level part in informational engineering - the hardware. In the classical software development we build our software, as stated, on top of several layers and standard libraries which deliver and prepare the necessary stuff. To get to know what the hardware, in this case the CPU, does after powered on we need to read about that in the reference manual.

The first hint is on **Chapter 2.3.2 Exception types** . The description for reset points out the following statement:

* *Reset is invoked on power up or a warm reset.*

That is quite good, so now we know that the processor after power up jumps to an adress written in the Reset Entry. Let us see, how we can teach the CPU to find our own reset function!

Therefore we go one step further to  **Chapter 2.3.4 Vector Table**. It may be good when you open the manual in parallel on page 40 - here we only look at a little extraction of the vector table to keep things easy.

| Offset Adress | Type |
|---------------|------|
| 0x4        | Reset|
| 0x0        | Initial Stackpointer Value|

Now we learned that the information, or more precisely, the adress of the reset function is located somewhere with an offset of 0x4.
 
Another interesting point is the second entry - here we need to tell the cpu, where the stackpointer points to after startup. More on that soon. First we has to clarify, where the exact adress is instead of the offset. Reading further in the manual we obtain this information:

* *On system reset, the vector table is fixed at address 0x00000000.*

In other words - the vector table lives at the very first start of the targets device memory. If our useable memory starts at e.g 0x5000, then the Initial Stackpointer Value is located at 0x5000 + 0x0 and the Reset entry at 0x5000 + 0x4. To do something useful with this information we now take a look at the memory.

### Memory System

In general, the memory of the STM32F303 Disco board is splitted into two parts - a non volatile Flashmemory and a SRAM. The flash contains all persistent data like programm
code, whereas the SRAM is used for dynamic stacks or heap memory. If you would place your stack into flash memory it would be destroyed soon because it is not designed for
so much read- and write operations. 

![memorysize](assets/01_mem_size.png)
[Fig. 1: Memorysize of different Boardrevisions][MemoryOverview]

The actual size depends on the board revision, in our case it is 128kbyte flash and 40kbyte sram memory. The next step is to determine how the cpu adresses this memory, so let's check out the STM32F303 documentation manual. On page 65 we get the following
entry:

| Flash area  | Flash memory addresses |
|---------------|------|
| Main memory                 | **0x0800 0000** - 0x0800 07FF |
| Main memory       | 0x0800 0800 - 0x0800 0FFF |
|                   | ... |

The bold adress is where the flash memory starts. The SRAM Block adress depends on the cpu, so in the CortexM4 manual on page 28 we see the memory adress band overview - the RAM starts at **0x20000000** .


These are the memory boundaries of the hardwareplatform:

| Memory area  | Size | Startaddress | Endaddress
|--------------|------|-------------|-----------|
| Flash        | 128k | 0x0800 0000 | 0x0801 F400 |
| SRAM         |  40k | 0x0200 0000 | 0x0200 9C40 |


Now we have enough background to write the first bootup code lines!

## Startup Code

### Rustify
We gonna split up the code into a startup part and a runtime part, where we can build up our kernel on top later on.

Step 1: Create the runtime library

``` cargo new runtime --lib ```

The CPU searches after power on a reset function, so lets define the most simple use case:

```rust
#![no_std]

// runtime/src/lib.rs
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {

    // don't worry about that for now
    extern "Rust" {
        fn main() -> !;
    }

    // don't worry about that for now
    main()
}
```
[Code 1: Reset Function - lib.rs][lib.rs]


There are three important things to consider:
* The function has to be **extern**, otherwise the compiler would delete it from the
 target binary because it gets never called directly.
* The return type has to be divergent because the function has no return point to jump back - remember, the call to this function is more or less hardcoded in hardware!
* #[no_mangle] is a directive for the linker script (more on that soon), that the 
symbol name not may changed or optimized. 

Also we add the #![no_std] directive to prevent the compiler to build in the standard
library into our binary, what would doesn't make sense because this library depends on
operating system features. Aside of that it would inflate our binary.

You remember from the last chapter, that the CPU expects a vector table containing vital adresses to jump to when starting. From rust side we can provide such an abstraction by adding the following line to the sourcecode:

```rust
// runtime/src/lib.rs

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

```
[Code 2: Vector Table Entry - lib.rs][lib.rs]


Here we define the symbol **RESET_VECTOR**, pointing to our created Reset function.
The unknown line __#[link_section = ".vector_table.reset_vector"]__ defines a section name, which we need later on to place in memory in the linker script. The given name is not mandatory, you can choose whatever is comfortable.

The source code of our runtime crate is almost done, except the obligatory panic handler, otherwise rust will not compile this code.

```
use core::panic::PanicInfo;

// ... 


// runtime/src/lib.rs
// just a generic panic handler
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
```
[Code 3: Panic Handler - lib.rs][lib.rs]


Add this to cargo.toml to tell rust what to do in case of panic:

```rust
// runtime/cargo.toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

[Code 4: Panic behaviour - cargo.toml][cargo.toml]

To make the buildprocess more comfortable, please add the following code into 
runtime:

```rust
// runtime/build.rs
use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put `link.x` in the build directory
    File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;

    Ok(())
}
```

[Code 5: Build automatisation - build.rs][build.rs]


Well done, the rust side from the runtime is almost done! What's left is the linker script and some ram initialization, but you have to be a little bit more patient to get to know that stuff.

Now jump out of this directoy and create a new rust crate:

``` cargo new os_kernel ```

This crate becomes our kernel, which will contain e.g. device peripherials or the scheduler. This time we only create a simple function with an indefinite loop, to keep the CPU busy. Add this code into your main.rs:

```rust
// os_kernel/src/main.rs
#![no_std]
#![no_main]

extern crate runtime;

#[no_mangle]
pub fn main() -> ! {
    loop {}
}

```

[Code 6: Target Function after booting - main.rs][main.rs]


This is now main function, which we call in the runtime program. To overcome compile errors, we need to tell the compiler, that we use our 'own' main function by adding #![no_main] to the top of the source code. Before we can check out the first build, we need to reference our runtime crate by adding the following line to the cargo.toml file:

```toml
os_kernel/cargo.toml 
[dependencies]
runtime = { path = "../runtime"}
```

[Code 7: Add dependency - cargo.toml][cargo.toml]

From the host side it is now clear what to do - the 'receipe' for the startup procedure is written. But we do not compile for our host system, but for the ARM target. We need to automate this process, so that we are able to type **cargo run** to trigger a crossbuild and flash procedure.

Therefore we need to create the following file:
```toml
// os_kernel/.cargo/config (DO NOT PASTE IN THIS COMMENT IN ORIGINAL FILE)
[target.thumbv7em-none-eabihf]
runner = "gdb-multiarch -q -x openocd.gdb"
rustflags = ["-C", "link-arg=-Tlink.x"]

[build]
target = "thumbv7em-none-eabihf"

```
[Code 8: Build target - config][config]

Also create two files in the given directory:
```sh
// os_kernel/openocd.cfg (DO NOT PASTE IN THIS COMMENT IN ORIGINAL FILE)
 Sample OpenOCD configuration for the STM32F3DISCOVERY development board

# Depending on the hardware revision you got you'll have to pick ONE of these
# interfaces. At any time only one interface should be commented out.

# Revision C (newer revision)
source [find interface/stlink-v2-1.cfg]

# Revision A and B (older revisions)
# source [find interface/stlink-v2.cfg]

source [find target/stm32f3x.cfg]
```
[Code 9: ST Link - openocd.cfg][openocd.cfg]

```
// os_kernel/openocd.gdb (DO NOT PASTE IN THIS COMMENT IN ORIGINAL FILE)
target extended-remote :3333
monitor reset halt
monitor arm semihosting enable 
load
continue

```
[Code 10: Connect the device to gdb - openocd.gdb][openocd.gdb]

Details are in the chapter **Development environment**.

Within this directory, we call now **cargo build** and see what happens:

``` error: couldn't read link.x: No such file or directory (os error 2) ```

Now it is time to write the linker script and bring the pieces of code together.

### Link it

The source code we have written before is fine, but when compiling it, the defined function "Reset" contains no information about its adress respectively its location. 
The linker script helps us to define more or less hardcoded **memory maps** and assign sections explicit to this memory locations. To understand this it is most suitable to start with it directly:

Please create the file **link.x** in the ``` runtime/ ``` directory. The mandatory part of any linker script is the definition of available memory, what looks like that:

```c
MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 128K
  SRAM (rwx) : ORIGIN = 0x20000000, LENGTH = 40K
}
```

These adresses you already know from the memory examination in the chapter before. Next we have to define the entry point of our program - remember, usually this is the main() function.

```
ENTRY(Reset);
```

Next we need a reference to the basic reset vector, which contains the initial stackpointer value and the adress of our reset function.

```
EXTERN(RESET_VECTOR);
```

Now it gets interesting, we gonna define where exactly the code pieces will live in the memory. The definition takes place in the section part of the script:

```
SECTIONS
{
    ...
}
```

The sections gets placed in memory as they are listed, that means, the first entry gets
placed at the start of memory. See this for an example:

```c
...
  .vector_table ORIGIN(FLASH) :
  {
    /* Entry 0: Initial SP Value. Placed at top end of SRAM and grows downwards. */
    LONG(ORIGIN(SRAM) + LENGTH(SRAM));
    
    /* Entry 1: Reset Function. Gets called after power up device. */
    KEEP(*(.vector_table.reset_vector));
  } > FLASH
...
```

We define a section called .vector_table which resides in the flash memory. The first
entry is the **endadress** of the RAM, where the downgrowing stack begins. **LONG** values are in this case 4 Byte respectively 32 Bit aligned. The second entry is 
the reset function. *KEEP* is in this context again a protection against deleting unreferenced symbols. The last keyword ``` flash ``` shifts the whole content into the flash memory. 

```c
  .text :
  {
    *(.text .text.*);
  } > FLASH
```

Text sections are predefined, it points to any code parts in the compiled binary.

```c
  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
```
This symbols refer to exception handling, but we will not use them in our binary to they get erased after compiling.


### First start

Now it is time for a first test. Please ensure you are in the directory ``` os_kernel ``` and then type in ``` cargo run ``` . Press **strg + c** to tell gdb to halt on the execution. When you see the following lines, everything went good:

![boot](assets/02_succesfull_boot.png)

[Fig. 2: Successfull booting][successfullBoot]

## Extensions

As you can see, right now we only have the vital parts and the .text section in the linker script. The .text section contains only executable code, to use const or static variables it is necessary to define some more predefined section so the compiler know where to place e.g. a const u32 variable.

To cover all **const** variables we add this to our linker script:

```c
SECTIONS
{
  ...
  ...
  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH
}
```

It is easy to remember when reading out this identifier - rodata stands for **read only data** .

To cover all **static** variables we add this to our linker script:

```c
SECTIONS
{
  ...
  ...
  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _dataStart = .;
    *(.data .data.*);
    _dataEnd = .;
  } > SRAM
}
```

As you can see, there is a predefined section only for static variables, whose content is known at compiletime. 




The last thing we need is a **.bss** section, where all uninitialized variables are stored. That could be for example something like this:

```rust
let i: u32;
```

You may have wondered why sometimes in programming languages like java uninitialized variables always have the value **0** . This is not 
coincidentally, on the contrary hardware is full of garbage values after power up because the state of the memory cells is not predictable.

In the next step we ensure that before the kernel may create any variables the ram is clean of that garbage values and fill the accordings segments with **0's**.


### Ram Initialization


src;

https://jsandler18.github.io/explanations/linker_ld.html

https://docs.rust-embedded.org/embedonomicon/preface.html

[MemoryOverview]: https://www.st.com/en/microcontrollers-microprocessors/stm32f303.html#overview (Memory Overview STM32F3x)

[main.rs]: https://github.com/wolfbiker1/NextLevelRTOS/tree/part1/boot_and_startup/kernel/src/main.rs (Github)
[lib.rs]: https://github.com/wolfbiker1/NextLevelRTOS/tree/part1/boot_and_startup/runtime/src/lib.rs (Github)
[build.rs]: https://github.com/wolfbiker1/NextLevelRTOS/tree/part1/boot_and_startup/runtime/build.rs (Github)
[cargo.toml]: https://github.com/wolfbiker1/NextLevelRTOS/tree/part1/boot_and_startup/runtime/Cargo.toml (Github)
[config]: https://github.com/wolfbiker1/NextLevelRTOS/tree/part1/boot_and_startup/kernel/.cargo/config (Github)
[openocd.cfg]: https://github.com/wolfbiker1/NextLevelRTOS/blob/part1/boot_and_startup/kernel/openocd.cfg (Github)
[openocd.gdb]: https://github.com/wolfbiker1/NextLevelRTOS/blob/part1/boot_and_startup/kernel/openocd.gdb (Github)

[successfullBoot]: assets/02_succesfull_boot.png (Picture)