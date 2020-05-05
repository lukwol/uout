mod mock;

#[cfg(test)]
mod serial {
    use super::mock::MockSerial;
    use embedded_hal::serial::Write;
    use ufmt::{derive::uDebug, uwrite, uwriteln};
    use uout::Output;

    #[test]
    fn writing_to_output() {
        let mut mock_serial = MockSerial::new();
        let mut out = mock_serial.output();

        uwrite!(out, "The quick {} fox ", "brown").unwrap();
        uwriteln!(out, "jumps over the {} dog", "lazy").unwrap();

        assert_eq!(
            mock_serial.captured_output.as_str(),
            "The quick brown fox jumps over the lazy dog\r\n"
        );
    }

    #[test]
    fn printing_struct() {
        #[derive(uDebug)]
        struct Pair {
            x: i32,
            y: i32,
        }

        let mut mock_serial = MockSerial::new();
        let mut out = mock_serial.output();
        let pair = Pair { x: 1, y: 2 };
        uwriteln!(out, "{:?}", pair).unwrap();

        assert_eq!(
            mock_serial.captured_output.as_str(),
            "Pair { x: 1, y: 2 }\r\n"
        );

        mock_serial.flush().unwrap();
        out = mock_serial.output();
        uwriteln!(out, "{:#?}", pair).unwrap();

        assert_eq!(
            mock_serial.captured_output.as_str(),
            "Pair {\r\n    x: 1,\r\n    y: 2,\r\n}\r\n"
        );
    }
}
