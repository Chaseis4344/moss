use crate::arch::Arch;
use libkernel::{
    CpuOps, VirtualMemory,
    error::Result,
    memory::address::{UA, VA},
};


mod cpu_ops;
mod virtual_memory;

fn extract_bit_to_bool(extract_from: u8, index: u8) -> bool {
    let mut bitmask: u8 = 1;

    bitmask = bitmask << index;
    let num = (extract_from & bitmask) >> index;

    num != 0
}

pub struct X86_64 {}
impl Arch for X86_64 {

    type UserContext = u8;
    
    fn name() -> &'static str {
        "x86-64"
    }

    fn power_off() -> ! {
        //Pull in things from acpi to accomplish
        todo!("Arch Impl");
    }

    fn new_user_context(entry_point: VA, stack_top: VA) -> Self::UserContext	{
        todo!("Arch Impl");
    }
    
    fn context_switch(new: Arc<Task>)	{
        todo!("Arch Impl");
    }
    
    fn create_idle_task() -> Task	{
        todo!("Arch Impl");
    }
    
    fn restart() -> !	{
        //Job for ACPI
        todo!("Arch Impl");
    }
    
    fn do_signal(
        sig: SigId,
        action: UserspaceSigAction,
    ) -> impl Future<Output = Result<<Self as Arch>::UserContext>>	{
        todo!("Arch Impl");
    }

    fn do_signal_return() -> impl Future<Output = Result<<Self as Arch>::UserContext>>	{
        todo!("Arch Impl");
    }

    unsafe fn copy_from_user(src: UA, dst: *mut (), len: usize)
    -> impl Future<Output = Result<()>>	{
        todo!("Arch Impl");
    }

    unsafe fn try_copy_from_user(src: UA, dst: *mut (), len: usize) -> Result<()>	{
        todo!("Arch Impl");
    }

    unsafe fn copy_to_user(src: *const (), dst: UA, len: usize)
    -> impl Future<Output = Result<()>>	{
        todo!("Arch Impl");
    }

    unsafe fn copy_strn_from_user(
        src: UA,
        dst: *mut u8,
        len: usize,
    ) -> impl Future<Output = Result<usize>>	{
        //Can probably grab strn from libc and call it a day for x86
        //after building out the interaction with userspace
        todo!("Arch Impl");
    }
}
