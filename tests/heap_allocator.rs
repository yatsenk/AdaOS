#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ada_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use ada_os::memory::{self, BootInfoFrameAllocator};
use ada_os::allocator;
use ada_os::allocator::HEAP_SIZE;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    ada_os::init(); 
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) }; 

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap Initialization Failed");

    test_main();
    ada_os::hlt_loop();
}

#[test_case]
fn basic_allocation() {
    let heap_val_1 = Box::new(42);
    let heap_val_2 = Box::new(13);
    assert_eq!(*heap_val_1, 42);
    assert_eq!(*heap_val_2, 13);
}

#[test_case]
fn vec_allocation() {
    let n = 1000;
    let mut vec = Vec::new();

    for i in 0..n {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes_allocation() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    ada_os::test_panic_handler(info)
} 
