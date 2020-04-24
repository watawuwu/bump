use std::result;

pub type AppError = failure::Error;
pub type Result<T> = result::Result<T, AppError>;
