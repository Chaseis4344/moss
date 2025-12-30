use libkernel::CpuOps;
use core::arch::asm;
use core::arch::x86_64::__cpuid;
use core::arch::x86_64::CpuidResult;

impl CpuOps for super::X86_64 {
    fn halt() -> ! {
        loop {
            x86_64::instructions::hlt()
        }
    }

    fn id() -> usize {
        unsafe { __cpuid(0x0B).edx as usize }
    }

    fn disable_interrupts() -> usize {
        let mut flags: u32;
        //Should be safe to push something to stack, pop it from stack and store into variable
        unsafe {
            //Should work in theory - valid on all x64 things
                asm!("pushfd", //flags to stack
                    "pop eax", //stack to reg
                    out("eax") flags
                ); //Get mask out
        }
        //Unsure on intention of this function, if we want to only extract the Interrupt flag from
        //EFlags, we can, but if we want to extract all flags, that is easier. I'll be assuming we
        //want the complete EFlags register for now and this can be changed later if that is not
        //the case
        //let if_bit = (flags & (1<<8));
        x86_64::instructions::interrupts::disable(); //Disable maskable interupts
        
        //Zero-extend if we can
        flags as usize
    }


    fn enable_interrupts() {
        //Set interrupt flag - does asm!("sti"); under the hood
        x86_64::instructions::interrupts::enable(); 
    }

    fn restore_interrupt_state(flags: usize) {
        
        unsafe {
            //Assumes EAX gets filled before execution of instructions
            asm!(
                "push eax",
                "popfd",
                in("eax") flags
            )
        }
        x86_64::instructions::interrupts::enable(); 
        
    }
}
