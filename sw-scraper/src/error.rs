#[derive(Debug)]
pub enum AppErrorType {
    _FetchError,
    _NotFound,
    _InvalidData,
    WriteError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}
