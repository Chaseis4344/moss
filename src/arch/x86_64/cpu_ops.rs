use libkernel::CpuOps;

impl CpuOps for super::x86_64 {
    fn halt() -> ! {
        loop {
            unsafe{
                asm!("hlt")
            }
        }
    }

    fn id() -> usize {
       //set to zero and pretend there is only 1 until x86 instructions are figured out 
       let id: usize = 0;
       id
    } 

    fn disable_interupts() -> usize {
        let interrupt_mask: usize 
        unsafe {
            // TODO: Add a feature check for 64-bit mode to make sure it works
            // and if it doesn't change lahf to use pushfd and popfd, and figure out how to
            // retrieve mask from stack
            //
            // This isn't perfect but should work in theory
            // We will get a CPU exception if this instruction isn't supported for a CPU
            let cpu_id_result: usize = 0;
            
            asm!("mov eax, 0x80000001");
            asm!("cpuid", out("ecx") cpu_id_result);
            if cpu_id_result == 1 {
                asm!("lahf", out("ah") interrupt_mask); //Get mask out
                asm!("cli"); //Disable maskable interupts
            } else {
                panic!("CPU does not support LAHF and SAHF")
            }

        }

        interrupt_mask //return mask
    }

    fn enable_interupts() {
        unsafe {
            asm!("sti") //Set interrupt flag
        }
    }

    fn restore_interupt_state(flags: usize) {
        unsafe {
            //Hoping that the asm macro loads registers before execution 
            // TODO: Add a feature check for 64-bit mode to make sure it works
            // and if it doesn't change lahf to use pushfd and popfd, and figure out how to
            let cpu_id_result: usize = 0;
            
            asm!("mov eax, 0x80000001");
            asm!("cpuid", out("ecx") cpu_id_result);
            if cpu_id_result ==1 {
                asm!("sahf", in("ah") flags)
                asm!("sti")
            }else {
                panic!("CPU does not support LAHF and SAHF")
            }
        }
    }
}
