//! Length-prefixed reading and writing
//!
//! Implements a popular format for sending data packets: First, a fixed width
//! integer in a fixed byte order is sent with the length of the data,
//! followed by the actual data.
use byteorder::{ReadBytesExt, WriteBytesExt, ByteOrder};
use std::io;

/// Implements sending of byte-slices with a length prefix.
pub trait LengthWriteExt : io::Write {
    fn write_u8_prefixed(&mut self, data: &[u8]) ->
        io::Result<()>;
    fn write_u16_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
        io::Result<()>;
    fn write_u32_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
        io::Result<()>;
    fn write_u64_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
        io::Result<()>;
}

/// Implements reading length-prefixed data.
pub trait LengthReadExt : io::Read {
    fn read_u8_prefixed(&mut self, buf: &mut Vec<u8>) ->
        io::Result<u8>;
    fn read_u16_prefixed<T: ByteOrder>(&mut self, buf: &mut Vec<u8>) ->
        io::Result<u16>;
    fn read_u32_prefixed<T: ByteOrder>(&mut self, buf: &mut Vec<u8>) ->
        io::Result<u32>;
    fn read_u64_prefixed<T: ByteOrder>(&mut self, buf: &mut Vec<u8>) ->
        io::Result<u64>;
}

impl<W: io::Write> LengthWriteExt for W {
    fn write_u8_prefixed(&mut self, data: &[u8]) ->
    io::Result<()> {
        try!(self.write_u8(data.len() as u8));
        try!(self.write_all(&data));

        Ok(())
    }

    fn write_u16_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
    io::Result<()> {
        try!(self.write_u16::<T>(data.len() as u16));
        try!(self.write_all(&data));

        Ok(())
    }

    fn write_u32_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
    io::Result<()> {
        try!(self.write_u32::<T>(data.len() as u32));
        try!(self.write_all(&data));
        Ok(())
    }

    fn write_u64_prefixed<T: ByteOrder>(&mut self, data: &[u8]) ->
    io::Result<()> {
        try!(self.write_u64::<T>(data.len() as u64));
        try!(self.write_all(&data));

        Ok(())
    }
}

impl<R: io::Read> LengthReadExt for R {
    fn read_u8_prefixed(&mut self, mut buf: &mut Vec<u8>) -> io::Result<u8> {
        let len = try!(self.read_u8());

        // expand size of buffer to fit new data. this will hopefully not
        // shrink the vector
        buf.resize(len as usize, 0);

        try!(self.read_exact(&mut buf));
        Ok(len)
    }

    fn read_u16_prefixed<T: ByteOrder>(&mut self, mut buf: &mut Vec<u8>) ->
    io::Result<u16> {
        let len = try!(self.read_u16::<T>());

        // expand size of buffer to fit new data. this will hopefully not
        // shrink the vector
        buf.resize(len as usize, 0);

        try!(self.read_exact(&mut buf));
        Ok(len)
    }

    fn read_u32_prefixed<T: ByteOrder>(&mut self, mut buf: &mut Vec<u8>) ->
    io::Result<u32> {
        let len = try!(self.read_u32::<T>());

        // expand size of buffer to fit new data. this will hopefully not
        // shrink the vector
        buf.resize(len as usize, 0);

        try!(self.read_exact(&mut buf));
        Ok(len)
    }

    fn read_u64_prefixed<T: ByteOrder>(&mut self, mut buf: &mut Vec<u8>) ->
    io::Result<u64> {
        let len = try!(self.read_u64::<T>());

        // expand size of buffer to fit new data. this will hopefully not
        // shrink the vector
        buf.resize(len as usize, 0);

        try!(self.read_exact(&mut buf));
        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use length_prefixed::{LengthWriteExt, LengthReadExt};
    use byteorder::{BigEndian, LittleEndian};

    #[test]
    fn test_u8_prefixed_write() {
        let mut buf = Vec::new();
        let expected = vec![5, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u8_prefixed(b"abcde").unwrap();

        assert_eq!(expected, buf);

        let mut out = Vec::new();
        buf.as_slice().read_u8_prefixed(&mut out).unwrap();
        assert_eq!(out, b"abcde");
    }

    #[test]
    fn test_u16_prefixed_write_be() {
        let mut buf = Vec::new();
        let expected = vec![0, 5, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u16_prefixed::<BigEndian>(b"abcde").unwrap();

        assert_eq!(expected, buf);

        let mut out = Vec::new();
        buf.as_slice().read_u16_prefixed::<BigEndian>(&mut out).unwrap();
        assert_eq!(out, b"abcde");
    }

    #[test]
    fn test_u32_prefixed_write_be() {
        let mut buf = Vec::new();
        let expected = vec![0, 0, 0, 5, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u32_prefixed::<BigEndian>(b"abcde").unwrap();

        assert_eq!(expected, buf);

        let mut out = Vec::new();
        buf.as_slice().read_u32_prefixed::<BigEndian>(&mut out).unwrap();
        assert_eq!(out, b"abcde");
    }

    #[test]
    fn test_u64_prefixed_write_be() {
        let mut buf = Vec::new();
        let expected = vec![0, 0, 0, 0, 0, 0, 0, 5,
                            0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u64_prefixed::<BigEndian>(b"abcde").unwrap();

        assert_eq!(expected, buf);

        let mut out = Vec::new();
        buf.as_slice().read_u64_prefixed::<BigEndian>(&mut out).unwrap();
        assert_eq!(out, b"abcde");
    }

    #[test]
    fn test_u16_prefixed_write_le() {
        let mut buf = Vec::new();
        let expected = vec![5, 0, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u16_prefixed::<LittleEndian>(b"abcde").unwrap();

        assert_eq!(expected, buf);

        let mut out = Vec::new();
        buf.as_slice().read_u16_prefixed::<LittleEndian>(&mut out).unwrap();
        assert_eq!(out, b"abcde");
    }

    #[test]
    fn test_u32_prefixed_write_le() {
        let mut buf = Vec::new();
        let expected = vec![5, 0, 0, 0, 0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u32_prefixed::<LittleEndian>(b"abcde").unwrap();

        assert_eq!(expected, buf);

        let mut out = Vec::new();
        buf.as_slice().read_u32_prefixed::<LittleEndian>(&mut out).unwrap();
        assert_eq!(out, b"abcde");
    }

    #[test]
    fn test_u64_prefixed_write_le() {
        let mut buf = Vec::new();
        let expected = vec![5, 0, 0, 0, 0, 0, 0, 0,
                            0x61, 0x62, 0x63, 0x64, 0x65];
        buf.write_u64_prefixed::<LittleEndian>(b"abcde").unwrap();

        assert_eq!(expected, buf);

        let mut out = Vec::new();
        buf.as_slice().read_u64_prefixed::<LittleEndian>(&mut out).unwrap();
        assert_eq!(out, b"abcde");
    }
}
