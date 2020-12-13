// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2020 Sunwook Eom <sunwook5492@gmail.com>

use crate::{cpu, println};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(m) = info.message() {
        println!("\nKernel Panic: {}", m);
    } else {
        println!("\nKernel Panic");
    }

    cpu::wait_forever()
}
