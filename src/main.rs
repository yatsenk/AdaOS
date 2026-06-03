#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ada_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ada_os::memory::{init, translate_addr};
use ada_os::println;
use x86_64::VirtAddr;
use x86_64::structures::paging::Translate;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    ada_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { init(phys_mem_offset) };

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
