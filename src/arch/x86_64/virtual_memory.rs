use libkernel::VirtualMemory;
use super::X86_64;

impl VirtualMemory for X86_64 {

 type PageTableRoot = /* Type */;
 type ProcessAddressSpace = /* Type */;
 type KernelAddressSpace = /* Type */;
 const PAGE_OFFSET: usize = 42;

 fn kern_address_space() -> &'static SpinLockIrq<<Self as VirtualMemory>::KernelAddressSpace, Self> { 
     todo!() 
 }
}
