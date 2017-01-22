MEMORY
{
  RAM (xrw)     : ORIGIN = 0x20000000, LENGTH = 6K
  FLASH (rx)    : ORIGIN = 0x08000000, LENGTH = 32K
}

stack_size = 1024;

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
_stack_end = _stack_start - stack_size;
_heap_end = _stack_end;

ENTRY(_reset);

SECTIONS
{
  .text :
  {
    /* Vector Table */
    LONG(_stack_start);
    LONG(_reset + 1);
    KEEP(*(.rodata._EXCEPTIONS));

    /* Reset Handler */
    _reset = .;
    *(.text._reset);

    *(.text*);
    *(.rodata*);
    _etext = .;
  } > FLASH

  .ARM.extab : /* debug info */
  {
    *(.ARM.extab*)
  } > FLASH

  /* Used by startup to initialize data */
  _sidata = .;

  /* Initialized data section */
  .data : AT (_sidata)
  {
    . = ALIGN(4);
    _sdata = .;     /* Create global symbol at start of data */
    *(.data)
    *(.data*)

    . = ALIGN(4);
    _edata = .;
  } > RAM

  /* Uninitialized data section */
  . = ALIGN(4);
  .bss :
  { 
    /* Used by startup to initialize .bss section */
    _sbss = .;
    __bss_start__ = _sbss;
    *(.bss)
    *(.bss*)
    *(COMMON)

    . = ALIGN(4);
    _ebss = .; /* Define a global symbol at bss end */
    __bss_end__ = _ebss;
  } > RAM

  . = ALIGN(4);
  .heap :
  {
    _heap_start = .;
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}

