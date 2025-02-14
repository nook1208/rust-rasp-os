// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2020-2021 Sunwook Eom <sunwook5492@gmail.com>

//! System console.

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// Console interfaces.
pub mod interface {
    use core::fmt;

    /// Console write functions.
    pub trait Write {
        /// Write a single character.
        fn write_char(&self, c: char);

        /// Write a Rust format string.
        fn write_fmt(&self, args:fmt::Arguments) -> fmt::Result;
    }

    /// Console read functions.
    pub trait Read {
        /// Read a single character.
        fn read_char(&self) -> char {
            ' '
        }
    }

    /// Console statistics
    pub trait Statistics {
        ///Return the number of characters written.
        fn chars_written(&self) -> usize {
            0
        }

        /// Return the number of characters read.
        fn chars_read(&self) -> usize {
            0
        }
    }

    /// Trait alias for a full-fledged console.
    pub trait All = Write + Read + Statistics;
}
