//! Platform agnostic, effortless writable outputs generation

#![no_std]

/// Serial (USART) implementation
pub mod serial;

/// Generic trait for output generation
pub trait Output<'a, T> {

    /// Returns writable output
    fn output(&'a mut self) -> T;
}
