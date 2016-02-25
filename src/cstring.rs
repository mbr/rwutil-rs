use std::{io, string};
use std::io::{Read};

pub trait Bytes0 : Read {
    fn read_cstring_raw(&mut self) -> io::Result<Vec<u8>> {
        let it = self.bytes();

        let result: Result<Vec<u8>,_> =
            it.take_while(|b| match *b {
                Ok(b) => b != 0,
                _ => true
            }).collect();

        Ok(try!(result))
    }
}

impl<R: io::Read> Bytes0 for R {
}

#[cfg(test)]
mod test {
    use cstring::Bytes0;

    fn test_cstring_raw() {
        let mut buf = vec![61, 62, 63, 0];
        assert_eq!(buf.as_slice().read_cstring_raw().unwrap(), b"abc")
    }
}
