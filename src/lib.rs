//! Platform agnostic, effortless writable outputs generation
//!
//! # Example
//!
//! ```
//! // Run Screen for `ttyACM0` with 115200 baud rate
//! // $ screen /dev/ttyACM0 115200
//!
//! # struct Serial;
//! #
//! # use embedded_hal::serial::Write;
//! #
//! # impl<'a> Write<u8> for Serial {
//! #     type Error = ();
//! #
//! #     fn flush(&mut self) -> nb::Result<(), Self::Error> {
//! #         Ok(())
//! #     }
//! #
//! #     fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
//! #         Ok(())
//! #     }
//! # }
//! #
//! use uout::Output;
//! use ufmt::{derive::uDebug, uwrite};
//!
//! # let mut serial = Serial;
//! #[derive(uDebug)]
//! struct Pair { x: u32, y: u32 }
//!
//! let mut out = serial.output();
//! let pair = Pair { x: 1, y: 2 };
//! uwrite!(out, "{:?}", pair).unwrap(); // prints: Pair { x: 1, y: 2 }
//! ```

#![no_std]

/// Serial (USART) implementation
pub mod serial;

/// Generic trait for output generation
pub trait Output<'a, T> {
    /// Returns writable output
    fn output(&'a mut self) -> T;
}
