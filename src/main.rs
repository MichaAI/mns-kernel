#![no_std]
#![no_main]

use core::panic::PanicInfo;

const HELLO: &[u8] = b"Hello Rust Kernel";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buff = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buff.offset(i as isize * 2) = byte;
            *vga_buff.offset(i as isize * 2 + 1) = 0xb;
        }
        
    }
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
