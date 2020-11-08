//! The `kernel` binary.

#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod memory;
mod panic_wait;
mod runtime_init;
mod console;
mod print;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    println!("[0] Hello This is eom");
    panic!("Kernel init is done.. So kill myself!!!");
}
