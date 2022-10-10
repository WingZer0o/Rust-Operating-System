#![no_std]

use core::panic::PanicInfo;

fn main() {
    
}


// need to have a panic handler in a no std environment
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
