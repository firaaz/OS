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
    use x86_64::registers::control::Cr3;

    println!("Hello World!");

    os::init();

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());


    #[cfg(test)]
    test_main();

    println!("no crashes!!!!");
    os::hlt_loop()
}
