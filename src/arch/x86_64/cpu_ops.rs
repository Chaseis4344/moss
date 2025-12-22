use libkernel::CpuOps;

impl CpuOps for super::x86_64 {
    fn halt() -> ! {
        loop {
            unsafe { asm!("hlt") }
        }
    }

    fn id() -> usize {
        //set to zero and pretend there is only 1 until x86 instructions are figured out
        let id: usize = 0;
        id
    }

    fn disable_interupts() -> usize {
        let mut interrupt_mask: usize;
        unsafe {
            // This isn't perfect but should work in theory
            // We will get a CPU exception if this instruction isn't supported for a CPU
            let mut cpu_id_result: usize = 0;

            asm!(
                "mov eax, 0x80000001",
                "cpuid",
                out("ecx") cpu_id_result
            );
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
            asm!("sti"); //Set interrupt flag
        }
    }

    fn restore_interupt_state(flags: usize) {
        unsafe {
            let mut cpu_id_result: usize = 0;

            asm!(
                "mov eax, 0x80000001",
                "cpuid",
                out("ecx") cpu_id_result
            );
            if cpu_id_result == 1 {
                //Hoping that the asm macro loads registers before execution
                asm!("sahf", in("ah") flags);
                asm!("sti");
            } else {
                panic!("CPU does not support LAHF and SAHF");
            }
        }
    }
}
