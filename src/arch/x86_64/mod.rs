use crate::memory::uaccess::UserCopyable;
use crate::process::owned::OwnedTask;
use crate::{arch::Arch, process::ctx};

use core::arch::x86_64::__cpuid;
use libkernel::error::KernelError;
use libkernel::{
    error::Result,
    memory::address::{UA, VA},
};

use crate::ProcessCtx;
use crate::process::Task;
use crate::process::thread_group::signal::SigId;
use crate::process::thread_group::signal::ksigaction::UserspaceSigAction;
use alloc::sync::Arc;

mod boot;
mod cpu_ops;
mod virtual_memory;

#[derive(Clone, Copy)]
pub struct UserContext {}

#[derive(Debug, Copy, Clone)]
pub struct x64PTraceGpRegs {}
pub struct X86_64 {}

impl UserContext {
    unsafe fn get_from_address(self, src: UA, len: usize) -> alloc::string::String {
        todo!("Get Items from userspace");
    }
}

unsafe impl UserCopyable for x64PTraceGpRegs {}

impl From<&UserContext> for x64PTraceGpRegs {
    fn from(value: &UserContext) -> Self {
        todo!("From UserContext");
    }
}

fn get_byte_from_userspace(byte: UA) -> Result<()> {
    todo!("");
}

impl Arch for X86_64 {
    type UserContext = UserContext;
    type PTraceGpRegs = x64PTraceGpRegs;
    //TODO: Figure out appropriate value
    const PAGE_OFFSET: usize = 64;

    fn name() -> &'static str {
        "x86-64"
    }

    fn power_off() -> ! {
        //Pull in things from acpi to accomplish
        todo!("Arch Impl");
    }

    fn restart() -> ! {
        //Job for ACPI
        todo!("Arch Impl");
    }

    fn new_user_context(entry_point: VA, stack_top: VA) -> Self::UserContext {
        todo!("Arch Impl");
    }

    fn context_switch(new: Arc<Task>) {
        todo!("Arch Impl");
    }

    fn create_idle_task() -> OwnedTask {
        todo!("Arch Impl");
    }

    async fn do_signal(
        ctx: ProcessCtx,
        sig: SigId,
        action: UserspaceSigAction,
    ) -> Result<<Self as Arch>::UserContext> {
        Err(KernelError::Other("Not Implented Yet"))
    }

    async fn do_signal_return(ctx: ProcessCtx) -> Result<UserContext> {
        Err(KernelError::Other("Not Implented Yet"))
    }

    unsafe fn copy_from_user(
        src: UA,
        dst: *mut (),
        len: usize,
    ) -> impl Future<Output = Result<()>> {
        //C analog to doing pointer arithmatic, need to find better way
        // async move {
        //     for i in 0..len {
        //         unsafe {
        //             *(dst.byte_add(i)) = get_byte_from_userspace(src)?;
        //         }
        //     }
        //     Ok(())
        // }
        async { todo!() }
    }

    unsafe fn try_copy_from_user(src: UA, dst: *mut (), len: usize) -> Result<()> {
        todo!("Arch Impl");
    }

    unsafe fn copy_to_user(
        src: *const (),
        dst: UA,
        len: usize,
    ) -> impl Future<Output = Result<()>> {
        async { todo!("Arch Impl") }
    }

    async unsafe fn copy_strn_from_user(src: UA, dst: *mut u8, len: usize) -> Result<usize> {
        //Can probably grab strn from libc and call it a day for x86
        //after building out the interaction with userspace
        todo!();
    }

    fn cpu_count() -> usize {
        // This should return logical cores, if true cores are wanted we will need more complex logic
        // SAFETY: This operation is stanard arcoss most manufacturers, Intel being a notable, and
        // new exception, this code can be revised later to comply with their system for logical
        // procesors
        unsafe { ((__cpuid(1).ebx >> 16) & 0xff) as usize }
    }

    fn get_cmdline() -> Option<alloc::string::String> {
        todo!("implement get_cmdline")
    }
}
