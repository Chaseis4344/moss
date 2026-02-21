use crate::{
    arch::Arch
};
use crate::process::thread_group::signal::ksigaction::UserspaceSigAction;
 use crate::process::thread_group::signal::SigId;
use crate::process::owned::OwnedTask;
use crate::process::Task;
use libkernel::error::KernelError;
use ringbuf::Arc;
use core::arch::x86_64::__cpuid;
use core::error::Error;
use libkernel::{
    memory::{
        address::{
            UA,
            VA,
        },
    }
};
use crate::memory::uaccess::UserCopyable;

mod cpu_ops;
mod virtual_memory;
mod boot;

#[derive(Clone,Copy)]
pub struct UserContext{

}

impl UserContext {
    unsafe fn get_from_address(self, src: UA, len: usize) -> alloc::string::String {
        todo!("Get Items from userspace");
    }
}
#[derive(Debug, Copy, Clone)]
pub struct x64PTraceGpRegs {}

unsafe impl UserCopyable for x64PTraceGpRegs {

}

impl From<&UserContext> for x64PTraceGpRegs {
    fn from(value: &UserContext) -> Self {
        todo!("From UserContext");
    }
}

pub struct X86_64 {}
impl Arch for X86_64 {
    type PTraceGpRegs = x64PTraceGpRegs;
    type UserContext = UserContext;
    
    fn name() -> &'static str {
        "x86-64"
    }

    fn power_off() -> ! {
        //Pull in things from acpi to accomplish
        todo!("Arch Impl");
    }

    fn restart() -> !	{
        //Job for ACPI
        todo!("Arch Impl");
    }
    
    fn new_user_context(entry_point: VA, stack_top: VA) -> Self::UserContext	{
        todo!("Arch Impl");
    }
    
    fn context_switch(new: Arc<Task>)	{
        todo!("Arch Impl");
    }
    
    fn create_idle_task() -> OwnedTask	{
        todo!("Arch Impl");
    }
    fn do_signal(
        sig: SigId,
        action: UserspaceSigAction,
    ) -> impl Future<Output = Result<<Self as Arch>::UserContext, KernelError>>	{
        async {todo!("Arch Impl");}
    }

    fn do_signal_return() -> impl Future<Output = Result<<Self as Arch>::UserContext, KernelError>>	{
        async {todo!("Arch Impl");}
    }

    unsafe fn copy_from_user(src: UA, dst: *mut (), len: usize)
    -> impl Future<Output = Result<(), KernelError>>	{

        async {todo!("Arch Impl");}
    }

    unsafe fn try_copy_from_user(src: UA, dst: *mut (), len: usize) -> Result<(), KernelError>	{
        todo!("Arch Impl");
    }

    unsafe fn copy_to_user(src: *const (), dst: UA, len: usize)
    -> impl Future<Output = Result<(), KernelError>>	{
        async {todo!("Arch Impl");}
    }

    unsafe fn copy_strn_from_user(
        src: UA,
        dst: *mut u8,
        len: usize,
    ) -> impl Future<Output = Result<usize, KernelError>>	{
        //Can probably grab strn from libc and call it a day for x86
        //after building out the interaction with userspace
        async {
           todo!(); 
        }
    }
    fn cpu_count() -> usize {
        // This should return logical cores, if true cores are wanted we will need more complex logic
        // SAFETY: This operation is stanard arcoss most manufacturers, Intel being a notable, and
        // new exception, this code can be revised later to comply with their system for logical
        // procesors
        ((__cpuid(1).ebx >> 16) & 0xff) as usize 
    }
}
