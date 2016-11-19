MEMORY
{
  RAM (xrw)     : ORIGIN = 0x20000000, LENGTH = 6K
  FLASH (rx)    : ORIGIN = 0x08000000, LENGTH = 32K
}

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    KEEP(*(.reset))
    KEEP(*(.exceptions))
  } > FLASH

  .text :
  {
    *(.text*)
  } > FLASH

  .rodata :
  {
    *(.rodata*)
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
