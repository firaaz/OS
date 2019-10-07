#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]

#![no_std]                  // Rust standard library is not linked
#![no_main]                 // disable the main() entry point

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
// use os::println;

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

// Function called on panic
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info);
}

// disable the mangling of the name so that is can be called by the linker as
// the linker looks for a _start by default
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}
