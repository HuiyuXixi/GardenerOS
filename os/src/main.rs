#![no_std]
#![no_main]
#[macro_use]
mod console;
mod sbi;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("entry.asm"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    loop{};
}
