// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2020 Sunwook Eom <sunwook5492@gmail.com>

//! BSP Memory Management.

/// The early boot core's stack address.
pub const BOOT_CORE_STACK_START: usize = 0x8_0000;
