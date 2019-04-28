pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ArqError(arq::error::Error),
    OsError(std::ffi::OsString),
    IoError(std::io::Error),
    OptionError,
    NotFound(String),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IoError(error)
    }
}

impl std::convert::From<std::ffi::OsString> for Error {
    fn from(error: std::ffi::OsString) -> Error {
        Error::OsError(error)
    }
}

impl std::convert::From<arq::error::Error> for Error {
    fn from(error: arq::error::Error) -> Error {
        Error::ArqError(error)
    }
}

// impl std::convert::From<std::option::NoneError> for Error {
//     fn from(_error: std::option::NoneError) -> Error {
//         Error::OptionError
//     }
// }
