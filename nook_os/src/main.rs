// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2020 Sunwook Eom <sunwook5492@gmail.com>

//! The `kernel` binary.

#![feature(naked_functions)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod memory;
mod panic_wait;
mod runtime_init;
mod console;
mod print;
mod synchronization;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    use console::interface::Statistics;

    println!("[0] Hello This is eom from pure Rust world !");
    println!("[1] Chars written: {}", bsp::console::console().chars_written());

    println!("[2] Stopping here !");

    cpu::wait_forever();
}
