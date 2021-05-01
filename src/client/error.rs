#[derive(Copy, Clone, Debug)]
pub enum AppErrorKind {
    IoInvalidData,

    ParseError,

    UnknownCommand,
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

pub type AppResult<T> = Result<T, AppError>;
