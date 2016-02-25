use std::{io, string};
use std::io::{Read};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8Error(string::FromUtf8Error)
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Error {
        Error::Utf8Error(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<Error> for io::Error {
    fn from(e: Error) -> io::Error {
        match e {
            Error::Io(err) => err,
            _ => unimplemented!()
        }
    }
}

trait Bytes0 : Read {
    fn read_cstring_utf8(&mut self) -> Result<String, Error> {
        let it = self.bytes();

        let result: Result<Vec<u8>,_> =
            it.take_while(|b| match *b {
                Ok(b) => b != 0,
                _ => true
            }).collect();

        Ok(try!(String::from_utf8(try!(result))))
    }
}

impl<R: io::Read> Bytes0 for R {
}
