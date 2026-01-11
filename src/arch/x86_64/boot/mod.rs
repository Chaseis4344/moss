use core::arch::global_asm;

//This will pull in everything and load into a multiboot long-mode enabled enviroment
global_asm!(include_str!("start.S"));

#[unsafe(no_mangle)]
pub extern "C" fn arch_stage_3() {
    //Should, again work in theory
    todo!("Establish alloc, larger stack, and call kmain")
} 
