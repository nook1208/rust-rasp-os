// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2020-2021 Sunwook Eom <sunwook5492@gmail.com>

//! Memory Management

use core::ops::RangeInclusive;

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Zero out an inclusive memory range.
///
/// # Safety
///
/// - `range.start` and `range.end` must be valid.
/// - `range.start` and `range.end` must be `T` aligned.
pub unsafe fn zero_volatile<T>(range: RangeInclusive<*mut T>)
where
    T: From<u8>,
{
    let mut ptr = *range.start();
    let end_inclusive = *range.end();

    while ptr <= end_inclusive {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}
