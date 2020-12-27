// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2020 Sunwook Eom <sunwook5492@gmail.com>

//! Device driver

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod bcm;
mod common;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use bcm::*;
