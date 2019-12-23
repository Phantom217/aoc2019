#[macro_use]
mod macros;

pub mod day01;
pub mod day02;

pub use self::reader::Reader;
pub use self::error::Error;

mod error {
    #[derive(Debug)]
    pub enum Error {
        Custom(String),
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Self::Custom(s) => write!(f, "{}", s),
            }
        }
    }

    impl std::error::Error for Error {}
}

mod reader {

    pub enum Reader<'a> {
        File(std::io::BufReader<std::fs::File>),
        Stdin(std::io::StdinLock<'a>)
    }

    impl<'a> std::io::Read for Reader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            match self {
                Self::File(file) => file.read(buf),
                Self::Stdin(guard) => guard.read(buf),
            }
        }
    }

    impl<'a> std::io::BufRead for Reader<'a> {
        fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
            match self {
                Self::File(reader) => reader.fill_buf(),
                Self::Stdin(guard) => guard.fill_buf(),
            }
        }

        fn consume(&mut self, amt: usize) {
            match self {
                Self::File(reader) => reader.consume(amt),
                Self::Stdin(guard) => guard.consume(amt),
            }
        }
    }
}
