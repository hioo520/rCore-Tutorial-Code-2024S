#![no_std]
#![no_main]
mod lang_items;
mod console;
pub mod sbi;
use crate::{console::print, sbi::shutdown};
core::arch::global_asm!(include_str!("entry.asm"));

pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
#[no_mangle]
pub fn rust_main() -> () {
    println!("Hello, world!");
    clear_bss();
    shutdown();
}
// qemu-system-riscv64 -machine virt -nographic -bios target/riscv64gc-unknown-none-elf/release/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
// qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os
