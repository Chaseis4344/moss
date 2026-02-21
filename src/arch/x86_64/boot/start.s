.global start
.extern long_mode_start #For coder understanding

.section .text
.code32

# Based on Phil Opp's multi-boot scripts

#
# Kernel Entry point: Sets up long-mode, then hands off to rust
#
start:
    mov esp, stack_top
   
    #after_xy labels are replacing returns for control flow, as I cannot trust rust to consistenly execute the ret instruction properly
    jmp check_multiboot # ensures we booted from a multiboot compliant bootloader (e.g. GRUB2)
    after_multiboot:
  
    jmp check_cpuid     # looks for cpuid support
    after_cpuid:

    jmp check_long_mode # looks for long mode support
    after_long_mode:

    jmp set_up_page_tables # Step 1 for long mode 
    after_tables:

    mov dword ptr [0xb8000], 0x2f4b2f4f
    jmp enable_paging     
    after_paging:
  
    mov dword ptr [0xb8004], 0x2f4b2f4f
    lgdt [gdt64_pointer]
    #The assembler will not let me use a long jump outside of att_syntax, so hacky workaround, use att syntax for the 1 line I need it in and exit back to intel syntax immediately
    .att_syntax
    ljmp $gdt64_code_seg, $long_mode_start
  .intel_syntax noprefix
check_multiboot:
    cmp eax, 0x36d76289
    jne no_multiboot
    jmp after_multiboot
no_multiboot:
    mov al, 0
    jmp error

check_cpuid:
    # Check if CPUID is supported by attempting to flip the ID bit (bit 21)
    # in the FLAGS register. If we can flip it, CPUID is available.

    # Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    # Copy to ECX as well for comparing later on
    mov eax, ecx

    # Flip the ID bit
    xor eax, 1 << 21

    # Copy EAX to FLAGS via the stack
    push eax
    popfd

    # Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    # Restore FLAGS from the old version stored in ECX (i.e. flipping the
    # ID bit back if it was ever flipped).
    push ecx
    popfd

    # Compare EAX and ECX. If they are equal then that means the bit
    # wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je no_cpuid
    jmp after_cpuid
no_cpuid:
    mov al, 1
    jmp error
check_long_mode:
    # test if extended processor info in available
    mov eax, 0x80000000    # implicit argument for cpuid
    cpuid                  # get highest supported argument
    cmp eax, 0x80000001    # it needs to be at least 0x80000001
    jb no_long_mode       # if it's less, the CPU is too old for long mode

    # use extended info to test if long mode is available
    mov eax, 0x80000001    # argument for extended processor info
    cpuid                  # returns various feature bits in ecx and edx
    test edx, 1 << 29      # test if the LM-bit is set in the D-register
    jz no_long_mode       # If it's not set, there is no long mode
    jmp after_long_mode
no_long_mode:
    mov al, 2
    jmp error
set_up_page_tables:
    # map first P4 entry to P3 table
    mov eax, p3_table
    or eax, 0b11 # present + writable
    mov [p4_table], eax


    # map first P3 entry to P2 table
    mov eax, p2_table
    or eax, 0b11 # present + writable
    mov [p3_table], eax
    
    mov ecx, 0         # counter variable

map_p2_table:
    # map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
    mov eax, 0x200000  # 2MiB
    mul ecx            # start address of ecx-th page
    or eax, 0b10000011 # present + writable + huge
    mov [p2_table + ecx * 8], eax# map ecx-th entry


    inc ecx            # increase counter
    cmp ecx, 512       # if counter == 512, the whole P2 table is mapped
    jne map_p2_table  # else map the next entry
    jmp after_tables

enable_paging:

    # load P4 to cr3 register (cpu uses this to access the P4 table)
    mov p4_table, eax
    mov eax, cr3

    # enable PAE-flag in cr4 (Physical Address Extension)
    mov edx, cr4
    or edx, 1 << 5
    mov cr4, edx
    # set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    # enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    mov dword ptr [0xb8000], 0x2f4b2f4f
    jmp after_paging

# Prints `ERR: ` and the given error code to screen and hangs.
# parameter: error code (in ascii) in al
error:
    mov dword ptr [0xb8000], 0x4f524f45
    mov dword ptr [0xb8004], 0x4f3a4f52
    mov dword ptr [0xb8008], 0x4f204f20
    mov byte  ptr [0xb800a], al
    hlt

.section .bss
.align 4096
p4_table:
    .skip 4096
p3_table:
    .skip 4096
p2_table:
    .skip 4096
stack_bottom:
    .skip 64
stack_top:


.section .rodata
gdt64_zero_entry:
  .quad 0 // zero entry

gdt64_code_entry:
  .set gdt64_code_seg, gdt64_code_entry - gdt64_zero_entry
  .quad (1<<44) | (1<<47) | (1<<43) | (1<<53) // code segment

gdt64_data_entry:
  .set gdt64_data_seg, gdt64_data_entry - gdt64_zero_entry
  .quad (1<<44) | (1<<47) | (1<<41) // data segment

gdt64_pointer:
  .set gdt64_pointer_seg, gdt64_data_entry - gdt64_pointer - 1
  .word gdt64_pointer_seg
  .quad gdt64_zero_entry
