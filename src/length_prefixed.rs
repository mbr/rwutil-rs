//! Length-prefixed stream packets
//!
//! Implements a popular format for sending data packets: First, a 32-bit
//! integer in big endian byte order is sent with the length of the data,
//! followed by the actual data.
use byteorder::{BigEndian, WriteBytesExt, ByteOrder};
use std::io;

use std::vec::Vec;

/// Allow sending of byte-slices with length prefix.
pub trait LengthWriteExt : io::Write {
    fn write_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
    io::Result< ()>;
}

impl<W: io::Write> LengthWriteExt for W {
    fn write_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
    io::Result< ()> {
        try!(self.write_u32::<T>(data.len() as u32));
        try!(self.write_all(&data));

        Ok(())
    }
}

mod test {
    use std::io::Write;
    use length_prefixed::LengthWriteExt;
    use byteorder::BigEndian;

    #[test]
    fn test_u32_write() {
        let mut buf = Vec::new();
        let expected = vec![0, 0, 0, 5, 0x61, 0x62, 0x63, 0x64, 0x65];
        //buf.write_prefixed();
        buf.write_prefixed::<BigEndian>(b"abcde");

        assert_eq!(&expected, &buf);
    }
}
