
ENTRY(start)

SECTIONS {

  /* Based on Phil Opperman's Rust Kernel multi-boot scripts */
    . = 1M;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        . = ALIGN(8);
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }
}
