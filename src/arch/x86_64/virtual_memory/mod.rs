use super::X86_64;
use linked_list_allocator::Heap;

use crate::sync::SpinLock;
use libkernel::error::KernelError;
use libkernel::memory::proc_vm::address_space::VirtualMemory;
use libkernel::memory::proc_vm::address_space::*;
use libkernel::memory::{address, page::PageFrame, paging::permissions::PtePermissions};

use alloc::vec::Vec;

use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};

mod allocater;

type VirtualAddress = libkernel::memory::address::Address<address::Virtual, ()>;

pub struct SpinlockHeap(pub SpinLock<Heap>);

pub struct AddrSpaceTempKern {}
pub struct AddrSpaceTempProcess {}

impl KernAddressSpace for AddrSpaceTempKern {
    fn map_mmio(
        &mut self,
        _physical_region: libkernel::memory::region::MemoryRegion<address::Physical>,
    ) -> Result<address::Address<address::Virtual, ()>, KernelError> {
        //CONTEXT: This function will map the Memory for Memory-mapped IO, in the context of the
        //boot window, this is a region of memory which will written to, for use IO Devices, like
        //most or all VGA Drivers on motherboards [0xb8000] for top left of screen in memory region 0xA0000-0xBFFFF
        todo!("Memmory Mapped IO Implementation Needed")
    }

    fn map_normal(
        &mut self,
        physical: libkernel::memory::region::MemoryRegion<address::Physical>,
        virt_range: libkernel::memory::region::MemoryRegion<address::Virtual>,
        perms: PtePermissions,
    ) -> Result<(), KernelError> {
        todo!("Memory Mapping");
    }
}

impl UserAddressSpace for AddrSpaceTempProcess {
    fn new() -> Result<Self, KernelError> {
        todo!()
    }

    fn activate(&self) {
        todo!()
    }

    fn deactivate(&self) {
        todo!()
    }

    fn map_page(
        &mut self,
        _page_frame: PageFrame,
        _: VirtualAddress,
        _pte_permissions: PtePermissions,
    ) -> Result<(), KernelError> {
        todo!()
    }

    fn unmap(&mut self, _address: VirtualAddress) -> Result<PageFrame, KernelError> {
        todo!()
    }

    fn remap(
        &mut self,
        _address: VirtualAddress,
        _: PageFrame,
        _: PtePermissions,
    ) -> Result<PageFrame, KernelError> {
        todo!()
    }

    fn protect_range(
        &mut self,
        _virtual_memory_region: libkernel::memory::region::MemoryRegion<address::Virtual>,
        _: PtePermissions,
    ) -> Result<(), KernelError> {
        todo!()
    }

    fn unmap_range(
        &mut self,
        _virtual_memory_region: libkernel::memory::region::MemoryRegion<address::Virtual>,
    ) -> Result<Vec<PageFrame>, KernelError> {
        todo!()
    }

    ///Take a Virtual Address and Return PageInfo if it exists
    fn translate(&self, _address: VirtualAddress) -> Option<PageInfo> {
        todo!()
    }

    fn protect_and_clone_region(
        &mut self,
        _: libkernel::memory::region::MemoryRegion<address::Virtual>,
        _: &mut Self,
        _: PtePermissions,
    ) -> Result<(), KernelError> {
        todo!()
    }
}

impl VirtualMemory for X86_64 {
    // Need to be defined at some point
    type PageTableRoot = u32;
    type ProcessAddressSpace = AddrSpaceTempProcess;
    type KernelAddressSpace = AddrSpaceTempKern;
    fn kern_address_space() -> &'static SpinLock<<Self as VirtualMemory>::KernelAddressSpace> {
        todo!("kern_address_space")
    }
}
