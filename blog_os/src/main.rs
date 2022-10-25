#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // name '_start' by default
    loop {}
}

// need to have a panic handler in a no std environment
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
