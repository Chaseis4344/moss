.global start
.section .text
.code32
start:
# print `OK` to screen
    mov dword ptr [0xb8000], 0x2f4b2f4f
    hlt
