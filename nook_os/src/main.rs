//! The `kernel` binary.

#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod memory;
mod panic_wait;
mod runtime_init;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    panic!()
}
