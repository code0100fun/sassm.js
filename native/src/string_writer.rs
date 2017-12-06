use std::string::String;
use std::io::Write;
use std::io::Error;
use std::ops::Deref;
use std::str::from_utf8;

pub struct StringWriter { pub value: String }

impl Deref for StringWriter {
    type Target = String;

    fn deref(&self) -> &String {
        &self.value
    }
}

impl Write for StringWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        let s = match from_utf8(bytes) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        self.value.push_str(s);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
