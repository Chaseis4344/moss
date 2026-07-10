use core::arch::{asm, global_asm};

//This will pull in everything and load into a multiboot long-mode enabled enviroment
//
global_asm!(include_str!("multiboot_header.s"));
global_asm!(include_str!("start.s"));
global_asm!(include_str!("secondary.s"));

//This will pull in everything and load into a multiboot long-mode enabled enviroment
// global_asm!(include_str!("start.S"));

#[unsafe(no_mangle)]
pub extern "C" fn arch_stage_3() {
    //Should, again work in theory
    //
    // ATTENTION: we have a very small stack and no guard page
    let VGA_BUFFER: *mut _ = 0xb8000 as *mut usize;
    const BK_GREEN: usize = 0x2f792f412f4b2f4f;
    unsafe {
        for i in 0..50 {
            *(VGA_BUFFER.add(i)) = 0 as usize;
        }
    };
    todo!("Establish alloc, larger stack, and call kmain");
    loop {}
}
