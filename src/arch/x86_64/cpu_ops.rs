use libkernel::CpuOps;
use core::arch::asm;
use core::arch::x86_64::__cpuid;
use core::arch::x86_64::CpuidResult;

impl CpuOps for super::X86_64 {
    fn halt() -> ! {
        loop {
            unsafe { asm!("hlt") }
        }
    }

    fn id() -> usize {
        __cpuid(0x0B).EDX 
    }

    fn disable_interrupts() -> usize {
        let mut interrupt_mask: i8;
        unsafe {
            // This isn't perfect but should work in theory
            // We will get a CPU exception if this instruction isn't supported for a CPU
            const LAHF_SUPPORT = 0x80000001;
            let cpu_id_result: CpuidResult = __cpuid(LAHF_SUPPORT);
            if cpu_id_result.ecx == 1 {
                asm!("lahf",
                    "mov ah al", 
                    out("al") interrupt_mask); //Get mask out
                asm!("cli"); //Disable maskable interupts
                // interrupt_mask = interrupt_mask >> 8; //Shift right 8 bits to get high register
                                                      //aligned with lower bits
            } else {
                panic!("CPU does not support LAHF and SAHF")
            }
        }

        interrupt_mask as usize //return mask
    }

    fn enable_interrupts() {
        unsafe {
            asm!("sti"); //Set interrupt flag
        }
    }

    fn restore_interrupt_state(flags: usize) {
        unsafe {
            let flags = flags as i8; // x86-64 binding only supports i8 for al
            const LAHF_SUPPORT = 0x80000001;
            let cpu_id_result: CpuidResult = __cpuid(LAHF_SUPPORT);
            if cpu_id_result.ecx == 1 {
                //Hoping that the asm macro loads registers before execution
                asm!("mov al ah",
                    "sahf", in("al") flags);
                asm!("sti");
            } else {
                panic!("CPU does not support LAHF and SAHF");
            }
        }
    }
}
