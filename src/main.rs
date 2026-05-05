#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ada_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ada_os::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    ada_os::init();

    let ptr = 0xeff as *mut u8;
    unsafe { *ptr = 42 }

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
