/* Main control flow of the program */

#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![allow(unconditional_recursion)]

#![no_std]                  // Rust standard library is not linked
#![no_main]                 // disable the main() entry point

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
// use os::println;

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

// Function called on panic
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os::memory::active_level_4_table;
    use os::memory::translate_addr;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::PageTable;

    println!("Hello World!");

    os::init();

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe {
        // active_level_4_table(phys_mem_offset)
    // };
//
    // for (i, entry) in l4_table.iter().enumerate() {
        // if !entry.is_unused() {
            // // println!("L4 Entry {}: {:?}", i, entry);
//
            // // to get the actual physical address
            // let phys = entry.frame().unwrap().start_address();
            // let virt = phys.as_u64() + boot_info.physical_memory_offset;
            // let ptr = VirtAddr::new(virt).as_mut_ptr();
            // let l3_table: &PageTable = unsafe {&*ptr};
//
            // for (i, entry) in l3_table.iter().enumerate() {
                // if !entry.is_unused() {
                    // println!("L3 Entry {}: {:?}", i, entry);
                    // let phys = entry.frame().unwrap().start_address();
                    // let virt = phys.as_u64() + boot_info.physical_memory_offset;
                    // let ptr = VirtAddr::new(virt).as_mut_ptr();
                    // let l3_table: &PageTable = unsafe {&*ptr};
//
                    // for (i, entry) in l3_table.iter().enumerate() {
                        // if !entry.is_unused() {
                            // println!("L2 Entry {}: {:?}", i, entry);
                            // let phys = entry.frame().unwrap().start_address();
                            // let virt = phys.as_u64() + boot_info.physical_memory_offset;
                            // let ptr = VirtAddr::new(virt).as_mut_ptr();
                            // let l3_table: &PageTable = unsafe {&*ptr};
//
                            // for (i, entry) in l3_table.iter().enumerate() {
                                // if !entry.is_unused() {
                                    // println!("L1 Entry {}: {:?}", i, entry);
                                // }
                            // }
                        // }
                    // }
                // }
            // }
        // }
    // }

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        0xb8000,                  // vga buffer
        0x201008,                 // random code page
        0x0100_0020_1a10,         // stack page
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe {
            translate_addr(virt, phys_mem_offset)
        };
        println!("{:?} -> {:?}", virt, phys)
    }

    #[cfg(test)]
    test_main();

    println!("no crashes!!!!");
    os::hlt_loop()
}
