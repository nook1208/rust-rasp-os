// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2020-2021 Sunwook Eom <sunwook5492@gmail.com>

//! The `kernel` binary.

#![feature(const_fn_fn_ptr_basics)]
#![feature(format_args_nl)]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

// `mod cpu` provides the `_start()` function, the first function to run. `_start()` then calls
// `runtime_init()`, which jumps to `kernel_init()`.

mod bsp;
mod console;
mod cpu;
mod driver;
mod memory;
mod panic_wait;
mod print;
mod runtime_init;
mod synchronization;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.
unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;

    for i in bsp::driver::driver_manager().all_device_drivers().iter() {
        if let Err(x) = i.init() {
            panic!("Error loading driver: {}: {}", i.compatible(), x);
        }
    }
    bsp::driver::driver_manager().post_device_driver_init();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    use console::interface::All;
    use driver::interface::DriverManager;

    // Wait for user to hit Enter.
    loop {
        if bsp::console::console().read_char() == '\n' {
            break;
        }
    }

    println!("[0] Booting on: {}", bsp::board_name());

    println!("[1] Drivers loaded:");
    for (i, driver) in bsp::driver::driver_manager()
        .all_device_drivers()
        .iter()
        .enumerate()
    {
            println!("{}. {}", i+1, driver.compatible());
    }

    println!("[2] Chars written: {}", bsp::console::console().chars_written());
    println!("[3] Echoing input now");

    loop {
        let c = bsp::console::console().read_char();
        bsp::console::console().write_char(c);
    }
}
