
MEMORY
{
  SRAM (rx) : ORIGIN = 0x2000233b, LENGTH = 204800
}

ENTRY(main);

SECTIONS
{
  .text :
  {
    KEEP(*(.main));
    *(.text .text.*);
  } > SRAM

  /* static variables or string literals need .bss .data and .rodata sections */

  /* .rodata is read only data; it is where global constants are placed. */
  .rodata :
  {
    *(.rodata .rodata.*);
  } > SRAM

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
