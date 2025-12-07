use libkernel::CpuOps;

impl CpuOps for super::x86_64 {
    fn halt() -> ! {
        loop {
            unsafe{
                naked_asm!{
                    "hlt"
                }
            }
        }
    }

    fn id() -> usize {
       //set to zero and pretend there is only 1 until x86 instructions are figured out 
        0
    } 

    fn disable_interupts() -> usize {

    }

    fn enable_interupts() {

    }

    fn restore_interupt_state(flags: usize) {

    }
}
