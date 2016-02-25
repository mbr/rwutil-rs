//! Length-prefixed stream packets
//!
//! Implements a popular format for sending data packets: First, a 32-bit
//! integer in big endian byte order is sent with the length of the data,
//! followed by the actual data.
use byteorder::{WriteBytesExt, ByteOrder};
use std::io;

/// Allow sending of byte-slices with length prefix.
pub trait LengthWriteExt : io::Write {
    fn write_u8_prefixed(&mut self, data: &[u8]) ->
        io::Result< ()>;
    fn write_u16_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
        io::Result< ()>;
    fn write_u32_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
        io::Result< ()>;
    fn write_u64_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
        io::Result< ()>;
}

impl<W: io::Write> LengthWriteExt for W {
    fn write_u8_prefixed(&mut self, data: &[u8]) ->
    io::Result< ()> {
        try!(self.write_u8(data.len() as u8));
        try!(self.write_all(&data));

        Ok(())
    }

    fn write_u16_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
    io::Result< ()> {
        try!(self.write_u16::<T>(data.len() as u16));
        try!(self.write_all(&data));

        Ok(())
    }

    fn write_u32_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
    io::Result< ()> {
        try!(self.write_u32::<T>(data.len() as u32));
        try!(self.write_all(&data));

        Ok(())
    }

    fn write_u64_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
    io::Result< ()> {
        try!(self.write_u64::<T>(data.len() as u64));
        try!(self.write_all(&data));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use length_prefixed::LengthWriteExt;
    use byteorder::{BigEndian, LittleEndian};

    #[test]
    fn test_u8_prefixed_write() {
        let mut buf = Vec::new();
        let expected = vec![5, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u8_prefixed(b"abcde").unwrap();

        assert_eq!(&expected, &buf);
    }

    #[test]
    fn test_u16_prefixed_write_be() {
        let mut buf = Vec::new();
        let expected = vec![0, 5, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u16_prefixed::<BigEndian>(b"abcde").unwrap();

        assert_eq!(&expected, &buf);
    }

    #[test]
    fn test_u32_prefixed_write_be() {
        let mut buf = Vec::new();
        let expected = vec![0, 0, 0, 5, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u32_prefixed::<BigEndian>(b"abcde").unwrap();

        assert_eq!(&expected, &buf);
    }

    #[test]
    fn test_u64_prefixed_write_be() {
        let mut buf = Vec::new();
        let expected = vec![0, 0, 0, 0, 0, 0, 0, 5,
                            0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u64_prefixed::<BigEndian>(b"abcde").unwrap();

        assert_eq!(&expected, &buf);
    }

    #[test]
    fn test_u16_prefixed_write_le() {
        let mut buf = Vec::new();
        let expected = vec![5, 0, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u16_prefixed::<LittleEndian>(b"abcde").unwrap();

        assert_eq!(&expected, &buf);
    }

    #[test]
    fn test_u32_prefixed_write_le() {
        let mut buf = Vec::new();
        let expected = vec![5, 0, 0, 0, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u32_prefixed::<LittleEndian>(b"abcde").unwrap();

        assert_eq!(&expected, &buf);
    }

    #[test]
    fn test_u64_prefixed_write_le() {
        let mut buf = Vec::new();
        let expected = vec![5, 0, 0, 0, 0, 0, 0, 0,
                            0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u64_prefixed::<LittleEndian>(b"abcde").unwrap();

        assert_eq!(&expected, &buf);
    }
}
