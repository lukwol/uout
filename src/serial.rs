use crate::Output;
use embedded_hal::serial;
use nb::block;
use ufmt::uWrite;

pub struct SerialOutput<'a, T>(&'a mut T);

impl<'a, T> Output<'a, SerialOutput<'a, T>> for T
where
    T: serial::Write<u8>,
{
    fn output(&'a mut self) -> SerialOutput<'a, T> {
        SerialOutput(self)
    }
}

impl<'a, T> uWrite for SerialOutput<'a, T>
where
    T: serial::Write<u8>,
{
    type Error = T::Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for byte in s.as_bytes() {
            if *byte == b'\n' {
                block!(self.0.write(b'\r'))?;
            }
            block!(self.0.write(*byte))?;
        }
        Ok(())
    }
}
