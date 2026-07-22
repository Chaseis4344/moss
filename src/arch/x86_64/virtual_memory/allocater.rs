use crate::sync::SpinLock;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};
use linked_list_allocator::Heap;

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
