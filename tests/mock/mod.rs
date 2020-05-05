use embedded_hal::serial;
use heapless::consts::U64;
use heapless::String;

pub(crate) struct MockSerial {
    pub(crate) captured_output: String<U64>,
}

impl MockSerial {
    pub(crate) fn new() -> Self {
        MockSerial {
            captured_output: String::new(),
        }
    }
}

impl<'a> serial::Write<u8> for MockSerial {
    type Error = ();

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.captured_output.clear();
        Ok(())
    }

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.captured_output.push(word as char).unwrap();
        Ok(())
    }
}
