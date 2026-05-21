#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ada_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ada_os::{memory::active_level_4_table, println};
use x86_64::{VirtAddr, structures::paging::PageTable};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    ada_os::init();

    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(physical_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
            
            let phys_addr = entry.frame().unwrap().start_address();
            let virt_addr = phys_addr.as_u64() + boot_info.physical_memory_offset;
            let ptr: *mut PageTable = VirtAddr::new(virt_addr).as_mut_ptr();
            let l3_table = unsafe { &*ptr };

            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("L3 Entry {}: {:?}", i, entry);

                    let phys_addr = entry.frame().unwrap().start_address();
                    let virt_addr = phys_addr.as_u64() + boot_info.physical_memory_offset;
                    let ptr: *mut PageTable = VirtAddr::new(virt_addr).as_mut_ptr();
                    let l2_table = unsafe { &*ptr };

                    for (i, entry) in l2_table.iter().enumerate() {
                        if !entry.is_unused() {
                            println!("L2 Entry {}: {:?}", i, entry);

                            let phys_addr = entry.frame().unwrap().start_address();
                            let virt_addr = phys_addr.as_u64() + boot_info.physical_memory_offset;
                            let ptr: *mut PageTable = VirtAddr::new(virt_addr).as_mut_ptr();
                            let l1_table = unsafe { &*ptr };

                            for (i, entry) in l1_table.iter().enumerate() {
                                println!("L1 Entry {}: {:?}", i, entry);
                            }
                        }
                    }
                }
            }
        }
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
