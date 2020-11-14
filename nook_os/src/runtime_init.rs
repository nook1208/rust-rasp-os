//! Rust runtime initialization code.
use crate::memory;
use core::ops::Range;

unsafe fn get_bss_range() -> Range<*mut usize> {
    extern "C" {
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    Range {
        start: &mut __bss_start,
        end: &mut __bss_end,
    }
}

unsafe fn zero_bss(){
    memory::zero_volatile(get_bss_range());
}

pub unsafe fn runtime_init() -> ! {
    zero_bss();

    crate::kernel_init()
}
