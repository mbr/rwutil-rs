use std::io;
use std::io::{Read, Write};

pub trait Bytes0Read : Read {
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

pub trait Bytes0Write : Write {
    fn write_cstring_raw(&mut self, data: &[u8]) -> io::Result<()> {
        try!(self.write_all(data));
        try!(self.write_all(&[0; 1]));
        Ok(())
    }
}

impl<R: Read> Bytes0Read for R {
}

impl<W: Write> Bytes0Write for W {
}

#[cfg(test)]
mod test {
    use cstring::{Bytes0Read, Bytes0Write};

    #[test]
    fn test_cstring_raw() {
        let buf = vec![0x61, 0x62, 0x63, 0];
        assert_eq!(buf.as_slice().read_cstring_raw().unwrap(), b"abc");

        let mut out = Vec::new();
        out.write_cstring_raw(b"abc").unwrap();
        assert_eq!(buf, out);
    }
}
