use crate::{Output, OutputConvertible};
use embedded_hal::serial;
use nb::block;
use ufmt::uWrite;

pub struct SerialOutput<T>(T);

impl<T> Output<T> for SerialOutput<T> {
    fn free(self) -> T {
        self.0
    }
}

impl<T> OutputConvertible<SerialOutput<T>> for T
where
    T: serial::Write<u8>,
{
    fn into_output(self) -> SerialOutput<T> {
        SerialOutput(self)
    }
}

impl<T> uWrite for SerialOutput<T>
where
    T: serial::Write<u8>,
{
    type Error = T::Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for byte in s.as_bytes() {
            let send = if *byte == b'\n' { b'\r' } else { *byte };
            block!(self.0.write(send))?;
        }
        Ok(())
    }
}
