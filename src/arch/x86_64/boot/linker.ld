
ENTRY(start)

SECTIONS {

  /* Based on Phil Opperman's Rust Kernel multi-boot scripts */
    . = 2M;
    . = ALIGN(8);

    .text :
    {
        *(.multiboot_header)
        *(.text .text.*)
    }
    .data : {
      *(.data .data.*)
    }
    .rodata : {
      *(.rodata .rodata.*)
    }
    .bss : {
      *(COMMON)
      *(.bss.bss.*)
    }
}
