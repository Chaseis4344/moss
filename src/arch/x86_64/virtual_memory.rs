use libkernel::VirtualMemory;
use super::X86_64;
use linked_list_allocator::Heap;

use crate::sync::SpinLock;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};

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

impl VirtualMemory for X86_64 {

// Need to be defined at some point
 type PageTableRoot = /* Type */ ;
 type ProcessAddressSpace = /* Type */;
 type KernelAddressSpace = /* Type */;
 const PAGE_OFFSET: usize = 42;

 fn kern_address_space() -> &'static SpinLockIrq<<Self as VirtualMemory>::KernelAddressSpace, Self> { 
     todo!("kern_address_space") 
 }
}
