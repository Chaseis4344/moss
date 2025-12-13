use super::Arch;
use libkernel::{
    CpuOps, VirtualMemory,
    error::Result,
    memory::address::{UA, VA},
};

mod cpu_ops;
mod virtual_memory;

pub struct x86_64 {}
impl Arch for x86_64 {
    fn name() -> &'static str {
        "x86_64"
    }

    fn power_off() -> ! {
        loop {}
    }
}
