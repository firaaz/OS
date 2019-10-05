#![no_std]                  // Rust standard library is not linked
#![no_main]                 // disable the main() entry point

use core::panic::PanicInfo;
mod vga_buffer;

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// disable the mangling of the name so that is can be called by the linker as
// the linker looks for a _start by default
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    loop {}
}
