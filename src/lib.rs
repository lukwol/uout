#![no_std]

pub mod serial;

pub trait OutputConvertible<T> where T: Output<Self>, Self:Sized {
    fn into_output(self) -> T;
}

pub trait Output<T> {
    fn free(self) -> T;
}
