#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
}

macro_rules! e {
    ($from:path, $to:ident) => {
        impl From<$from> for Error { fn from(value: $from) -> Self { Self::$to(value) } }
    }
}

e!(std::io::Error, IO);
