
/* generall memory info; manual cortex m4 page 30 */
MEMORY
{
  /* see page 59 at reference manual */
  /* flash area starts @0800000 */
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 512K

  /* also sram area starts @20000000 */
  /* Block first 255 bytes for tcb table */
  TCBMETA (rwx) : ORIGIN = 0x20000000, LENGTH = 4
  TCBPTR (rwx) : ORIGIN = 0x20000004, LENGTH = 32
  TCBBLK (rwx) : ORIGIN = 0x20000024, LENGTH = 32
  MEM (rwx) : ORIGIN = 0x20000044, LENGTH = 4027
  SRAM (rwx) : ORIGIN = 0x20000FFF, LENGTH = 33745
}

/* The Entry section expects the symbol name of the first executable 
piece of code which will be loaded into the processor. Logically it is
the first part of the .text section. */
ENTRY(Reset);

/* Because there is no direct reference to the reset fn
the linker assumes it is unused and will erase it from the
final binary. With the help of the 'extern' keyword we allow
that unresolved symbols may take place in the final binary so they
can survice the compile process. These functions are splitted 
into two symbol names because of their return type. See lib.rs 
for more info. */
EXTERN(RESET);
EXTERN(EXCEPTIONS);

/* Description of what the memory contains and how it will be located . */
SECTIONS
{
  /* According to cortex m4 reference manual page 40 the cortex m4 cpu expects the adresses of the predefined handlers
  at the beginning of memory. The first symbol is the .vector_table, which resides at ORIGIN(FLASH) : 0x08000000. */
  .vector_table ORIGIN(FLASH) :
  {
    /* Entry 0: Initial SP Value. Placed at top end of SRAM and grows downwards. */
    LONG(ORIGIN(SRAM) + LENGTH(SRAM));
    
    /* Entry 1: Reset Function. Gets called after power up device. */
    KEEP(*(.vector_table.reset));

    /* Entry 2: Any other exception handlers. */
    KEEP(*(.vector_table.exceptions));
  } > FLASH
  .text :
  {
    *(.text .text.*);
  } > FLASH

  /* static variables or string literals need .bss .data and .rodata sections */

  /* .rodata is read only data; it is where global constants are placed. */
  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .tcbmeta :
  {
    KEEP(*(.tcbmeta)) 
  } > TCBMETA

  .tcbptr :
  {
    KEEP(*(.tcbptr)) 
  } > TCBPTR

  .tcbblk :
  {
    KEEP(*(.tcbblk)) 
  } > TCBBLK

  /* .bss is where uninitialized global variables are placed. */
  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > SRAM

  /* .data is where global variables that are initialized at compile time are placed. */
  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > SRAM

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}
