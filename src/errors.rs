use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AppError {
    #[error("Failed to generate secret santa, please try to remove couples or add more people")]
    AttemptsLimitReached,
}
