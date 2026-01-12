use libkernel::VirtualMemory;
use super::X86_64;
use linked_list_allocator::Heap;
 use x86_64::structures::paging::{page::PageRange, PageTable};
use crate::sync::SpinLock;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};
 use libkernel::sync::spinlock::SpinLockIrq;
use libkernel::{UserAddressSpace,KernAddressSpace};
pub struct SpinlockHeap(pub SpinLock<Heap>);

#[global_allocator]
pub static HEAP_ALLOCATOR: SpinlockHeap = SpinlockHeap(SpinLock::new(Heap::empty()));

unsafe impl GlobalAlloc for SpinlockHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock_save_irq()
            .allocate_first_fit(layout)
            .ok()
            .map_or(core::ptr::null_mut(), |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.0
                .lock_save_irq()
                .deallocate(NonNull::new_unchecked(ptr), layout)
        }
    }
}

pub struct X86ArchProccessAddressSpace {}
pub struct X86ArchKernAddressSpace {}

impl VirtualMemory for X86_64 {

// Need to be defined at some point
 type PageTableRoot = PageTable;
 type ProcessAddressSpace = X86ArchProccessAddressSpace;
 type KernelAddressSpace = X86ArchKernAddressSpace;
 const PAGE_OFFSET: usize = 42;

 fn kern_address_space() -> &'static SpinLockIrq<<Self as VirtualMemory>::KernelAddressSpace, Self> { 
     todo!("kern_address_space") 
 }
}

impl UserAddressSpace for X86ArchProccessAddressSpace {
    fn new() -> libkernel::error::Result<Self>
        where
            Self: Sized {
        todo!()    
    }

    fn unmap(&mut self, va: libkernel::memory::address::VA) -> libkernel::error::Result<libkernel::memory::page::PageFrame> {
        todo!()
    }

    fn remap(&mut self, va: libkernel::memory::address::VA, new_page: libkernel::memory::page::PageFrame, perms: libkernel::memory::permissions::PtePermissions) -> libkernel::error::Result<libkernel::memory::page::PageFrame> {
        todo!()
    }
    
    fn activate(&self) {
        todo!()
    }

    fn map_page(&mut self, page: libkernel::memory::page::PageFrame, va: libkernel::memory::address::VA, perms: libkernel::memory::permissions::PtePermissions) -> libkernel::error::Result<()> {
        todo!()    
    }

    fn translate(&self, va: libkernel::memory::address::VA) -> Option<libkernel::PageInfo> {
        todo!()
    }

    fn deactivate(&self) {
        todo!()    
    }
    
    fn unmap_range(&mut self, va_range: libkernel::memory::region::VirtMemoryRegion) -> libkernel::error::Result<alloc::vec::Vec<libkernel::memory::page::PageFrame>> {
        todo!()
    }

    fn protect_range(&mut self, va_range: libkernel::memory::region::VirtMemoryRegion, perms: libkernel::memory::permissions::PtePermissions) -> libkernel::error::Result<()> {
        todo!()
    }

    fn protect_and_clone_region(
            &mut self,
            region: libkernel::memory::region::VirtMemoryRegion,
            other: &mut Self,
            perms: libkernel::memory::permissions::PtePermissions,
        ) -> libkernel::error::Result<()>
        where
            Self: Sized {
        todo!()
    }

}

impl KernAddressSpace for X86ArchKernAddressSpace {

        fn map_mmio(&mut self, region: libkernel::memory::region::PhysMemoryRegion) -> libkernel::error::Result<libkernel::memory::address::VA> {
            todo!()
        }
        fn map_normal(
                &mut self,
                phys_range: libkernel::memory::region::PhysMemoryRegion,
                virt_range: libkernel::memory::region::VirtMemoryRegion,
                perms: libkernel::memory::permissions::PtePermissions,
            ) -> libkernel::error::Result<()> {
            todo!()
        }
} 
