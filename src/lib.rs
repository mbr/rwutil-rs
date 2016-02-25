//! Read/Write traits for reading common formats like length-prefixed packets.
//!
//! `rwutils` handles a few common formats of binary data:
//!
//! * C-style strings, that is 0-terminated sequences of non-zero bytes inside
//!   the `bytes0` module. Check its docs for examples of reading Strings.
//! * Length prefixed packets, a u8/u16/u32/u64 of the number of bytes to
//!   read, followed by the actual data. See `length_prefix`.
//!
//! All readers are implemented as additional traits for the `io::Read` and
//! `io::Write`. After using them, additional methods become available on these
//! types.
//!
//! **Please read the notes regarding security** in module as well,
//! especially if you are handling untrusted user input.

extern crate byteorder;
pub mod bytes0;
pub mod length_prefixed;
