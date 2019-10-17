/* Main control flow of the program */

#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![allow(unconditional_recursion)]

#![no_std]                  // Rust standard library is not linked
#![no_main]                 // disable the main() entry point

#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
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
    use os::allocator;
    use os::memory;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::PageTable;
    use x86_64::structures::paging::MapperAllSizes;
    use x86_64::structures::paging::Page;
    use os::memory::BootInfoFrameAllocator;

    println!("Hello World!");

    os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());


    let refrence_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = refrence_counted.clone();
    println!("current refrence count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(refrence_counted);
    println!("refrence count is {}", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("no crashes!!!!");
    os::hlt_loop()
}
