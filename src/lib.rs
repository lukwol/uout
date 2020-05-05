#![no_std]

mod serial;

pub trait Output<'a, T> {
    fn output(&'a mut self) -> T;
}
