//! Read and write 0-terminated byte strings.
//!
//! Example:
//!
//! ```
//! use rwutil::bytes0::ReadBytes0Ext;
//!
//! // a &[u8] implements the io::Read interface
//! let src: Vec<u8> = vec![0x61, 0x62, 0x63, 0];
//!
//! // reading data into a new vector
//! let buf: Vec<u8> = src.as_slice().read_bytes0().unwrap();
//!
//! assert_eq!(buf, b"abc")
//! ```
//!
//! # Security
//! The current implementation is similar to the trouble `gets` from C in that
//! it does not have a maximum size for reading. While you will not encounter
//! a buffer overflow, the vector the string is read into can grow arbitrarily
//! large, allow a supplier of untrusted data to cause an OOM panic if allowed
//! to send an infinite amount of data.

use std::io;
use std::io::{Read, Write};

pub trait ReadBytes0Ext : Read {
    fn read_bytes0(&mut self) -> io::Result<Vec<u8>> {
        let it = self.bytes();

        let result: Result<Vec<u8>,_> =
            it.take_while(|b| match *b {
                Ok(b) => b != 0,
                _ => true
            }).collect();

        Ok(try!(result))
    }
}

pub trait WriteBytes0Ext : Write {
    fn write_bytes0(&mut self, data: &[u8]) -> io::Result<()> {
        try!(self.write_all(data));
        try!(self.write_all(&[0; 1]));
        Ok(())
    }
}

impl<R: Read> ReadBytes0Ext for R {
}

impl<W: Write> WriteBytes0Ext for W {
}

#[cfg(test)]
mod test {
    use bytes0::{ReadBytes0Ext, WriteBytes0Ext};

    #[test]
    fn test_bytes0() {
        let buf = vec![0x61, 0x62, 0x63, 0];
        assert_eq!(buf.as_slice().read_bytes0().unwrap(), b"abc");

        let mut out = Vec::new();
        out.write_bytes0(b"abc").unwrap();
        assert_eq!(buf, out);
    }
}
