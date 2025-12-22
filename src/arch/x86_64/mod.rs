use super::Arch;
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

    num as bool
}

pub struct x86_64 {}
impl Arch for x86_64 {
    fn name() -> &'static str {
        "x86_64"
    }

    fn power_off() -> ! {
        loop {}
    }
}
