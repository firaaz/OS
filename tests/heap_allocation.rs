use os::{serial_print, serial_print};
use alloc::boxed::Box;
use alloc::vec::Vec;

fn main(boot_info: &'static BootInfo) -> ! {
    use os::allocator;
    use os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

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
    test_main();
    loop{}
}

#[test_case]
fn simple_allocation() {
    serial_print!("simple_allocation...");
    let heap_value = Box::new(41);
    assert_eq!(*heap_value, 41);
    serial_print!("[ok]");
}

#[test_case]
fn large_vec() {
    serial_print!("large vector......");
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>()m (n - 1) * n / 2);
    serial_println!("[ok]");
}

#[test_case]
fn many_boxes() {
    serial_print!("many boxes.......");
    for i in 0..10_000 {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    serial_println!("[ok]");
}
