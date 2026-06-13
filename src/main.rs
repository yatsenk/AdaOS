#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ada_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;

use ada_os::memory::{self, BootInfoFrameAllocator, init, translate_addr};
use ada_os::allocator;
use ada_os::println;
use alloc::vec::Vec;
use x86_64::VirtAddr;
use x86_64::structures::paging::{Translate, Page};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

const BANNER: &str = r"
     _    ____    _       ___  ____  
    / \  |  _ \  / \     / _ \/ ___| 
   / _ \ | | | |/ _ \   | | | \___ \ 
  / ___ \| |_| / ___ \  | |_| |___) |
 /_/   \_\____/_/   \_\  \___/|____/ 
";

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("{}", BANNER);

    ada_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init(phys_mem_offset) };
    let mut frame_allocator = unsafe { 
        BootInfoFrameAllocator::init(&boot_info.memory_map) 
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");

    let x = Box::new(42);
    println!("heap value at {:p}", x);

    let y = Box::new(42);
    println!("heap value at {:p}", y);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_mapping(page, &mut mapper, &mut frame_allocator);

    #[allow(unused)]
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f324_d422_a323_af32); };

    let stack = 0u64;
    let stack_ptr = &stack as *const u64 as u64;
    let my_fn_ptr = translate_addr as *const () as u64;
    let adresses = [
        0xb8000,
        stack_ptr,  
        my_fn_ptr,
        boot_info.physical_memory_offset,
    ];

    for &addres in &adresses {
        let virt = VirtAddr::new(addres);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?} ", virt, phys);
    }
    
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    ada_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ada_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ada_os::test_panic_handler(info)
}
