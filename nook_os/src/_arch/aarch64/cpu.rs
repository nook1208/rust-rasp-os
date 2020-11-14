//! Architectural processor code.

use cortex_a::{asm, regs::*};
use crate::{bsp, cpu};

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// The entry of the `kernel` binary.
///
/// The function must be named `_start`, because the linker is looking for this exact name.
///
/// # Safety
///
/// - Linker script must ensure to place this function at `0x8_0000`.
#[naked]
#[no_mangle]
pub unsafe fn _start() -> ! {
    if bsp::cpu::BOOT_CORE_ID == cpu::smp::core_id() {
        SP.set(bsp::memory::BOOT_CORE_STACK_START as u64);
        crate::runtime_init::runtime_init();
    } else {
        wait_forever();
    }
}

/// Pause execution on the core.
#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe();
    }
}
