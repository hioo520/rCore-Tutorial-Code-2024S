use core::panic::PanicInfo;
use crate::print;
use crate::println;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Hello, world!");
    loop {}
}
