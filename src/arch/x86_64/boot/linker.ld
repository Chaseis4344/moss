ENTRY(start)

SECTIONS {
  /* Based on Phil Opperman's Rust Kernel multi-boot scripts */
    . = 2M;
    . = ALIGN(8);

    .boot : {
        *(.multiboot_header)
    }

    .text :
    {
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
      *(.bss .bss.*)
    }
}
