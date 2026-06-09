#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ada_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(_boot_info: &'static BootInfo) -> ! {
    unimplemented!();
}

fn panic_handler(info: &PanicInfo) -> ! {
    ada_os::test_panic_handler(info)
} 
