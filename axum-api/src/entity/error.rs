#[derive(Debug)]
pub enum AppError {
    InvalidInput,
    PasswordHashFailed,
    DatabaseOperationError,
}
