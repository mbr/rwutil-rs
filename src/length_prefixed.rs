//! Length-prefixed stream packets
//!
//! Implements a popular format for sending data packets: First, a 32-bit
//! integer in big endian byte order is sent with the length of the data,
//! followed by the actual data.
use byteorder::{BigEndian, WriteBytesExt};
use std::io;

/// Allow sending of byte-slices with length prefix.
pub trait LengthWriteExt : io::Write {
    fn send_length_prefixed(&mut self, data: &[u8]) -> io::Result<()>;
}

impl<W: io::Write> LengthWriteExt for W {
    fn send_length_prefixed(&mut self, data: &[u8]) -> io::Result<()> {
        try!(self.write_u32::<BigEndian>(data.len() as u32));
        try!(self.write_all(&data));

        Ok(())
    }
}
