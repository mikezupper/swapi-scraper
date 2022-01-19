#[derive(Debug)]
pub enum AppErrorType {
    _FetchError,
    _NotFound,
    _InvalidData,
    ConfigError,
    _WriteError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}
