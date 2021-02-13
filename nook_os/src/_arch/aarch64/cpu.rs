// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2020-2021 Sunwook Eom <sunwook5492@gmail.com>

//! Architectural processor code.

use cortex_a::{asm, regs::*};
use crate::{bsp, cpu};

//--------------------------------------------------------------------------------------------------
// Boot Code
//--------------------------------------------------------------------------------------------------

/// The entry of the `kernel` binary.
///
/// The function must be named `_start`, because the linker is looking for this exact name.
///
/// # Safety
///
/// - Linker script must ensure to place this function where it is expected by the target machine.
/// - We have to hope that the compiler omits any stack pointer usage before the stack pointer is
///   actually set (`SP.set()`).
#[no_mangle]
pub unsafe fn _start() -> ! {
    if bsp::cpu::BOOT_CORE_ID == cpu::smp::core_id() {
        SP.set(bsp::memory::boot_core_stack_end() as u64);
        crate::runtime_init::runtime_init();
    } else {
        wait_forever();
    }
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

pub use asm::nop;

/// Spin for `n` cycles.
#[cfg(feature = "bsp_rpi3")]
#[inline(always)]
pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        asm::nop();
    }
}

/// Pause execution on the core.
#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe();
    }
}
