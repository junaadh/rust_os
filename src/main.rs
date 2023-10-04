#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{memory, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    rust_os::init();

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("    L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // let addresses = [
    //     0xb8000,                          // the identity-mapped vga buffer page
    //     0x201008,                         // some code page
    //     0x0100_0020_1a10,                 // some stack page
    //     boot_info.physical_memory_offset, // virtual address mapped to physical address 0
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It hath not crashed works!");
    rust_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
