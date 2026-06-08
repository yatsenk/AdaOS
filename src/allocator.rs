use core::alloc::GlobalAlloc;
use core::ptr::null_mut;

pub struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        null_mut()
    }   

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        panic!("Should never called.")
    }
}

#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;
