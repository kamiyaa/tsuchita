use std::io;

#[derive(Copy, Clone, Debug)]
pub enum AppErrorKind {
    IoGeneric,
    IoInvalidData,

    ConnectionFailed,

    ParseError,

    UnknownCommand,
}

impl std::convert::From<io::ErrorKind> for AppErrorKind {
    fn from(err: io::ErrorKind) -> Self {
        match err {
            io::ErrorKind::InvalidData => Self::IoInvalidData,
            _ => Self::IoGeneric,
        }
    }
}

impl std::convert::From<ureq::Error> for AppErrorKind {
    fn from(err: ureq::Error) -> Self {
        Self::ConnectionFailed
    }
}

#[derive(Clone, Debug)]
pub struct AppError {
    _kind: AppErrorKind,
    _cause: String,
}

impl AppError {
    pub fn new(_kind: AppErrorKind, _cause: String) -> Self {
        AppError { _kind, _cause }
    }

    pub fn kind(&self) -> AppErrorKind {
        self._kind
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self._cause)
    }
}

impl std::convert::From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self {
            _kind: AppErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}

impl std::convert::From<ureq::Error> for AppError {
    fn from(err: ureq::Error) -> Self {
        Self {
            _kind: AppErrorKind::from(err),
            _cause: "Failed to connect".to_string(),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
