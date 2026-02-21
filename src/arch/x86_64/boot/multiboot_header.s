# Boot script needs to get us into long mode, assume GRUB2 enviroment

.extern start
.section .multiboot_header
header_start: 
    .long 0xe85250d6             # magic number (multiboot 2)
    .long 0                      # architecture 0 (protected mode i386)
    .long header_end - header_start
    # checksum
    .long 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
    
    # insert optional multiboot tags here
.align 8
  end_tag:
    # required end tag
    .word 0 # type
    .word 0 # flags
    .long end_tag - header_end # size
header_end: 

