#![no_std]                  // Rust standard library is not linked
#![no_main]                 // disable the main() entry point

use core::panic::PanicInfo;

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


static HELLO: &[u8] = b"Hello World!";

// disable the mangling of the name so that is can be called by the linker as
// the linker looks for a _start by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // The entry point as the linker looks for _start
    let vga_buffer = 0xb8000 as *mut u8; // the vga buffer is located at 0xb8000
                                         // and it is casted to a raw pointer

    for (i, &byte) in HELLO.iter().enumerate() {
        // unsafe as rust cant give any guarentee that the raw pointer points 
        // to any valid memory, hence unsafe to say that we are sure it works
        unsafe { 
            *vga_buffer.offset(i as isize * 2) = byte;    // write the string
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // coloring as cyan
        }
    }

    loop {}
}
