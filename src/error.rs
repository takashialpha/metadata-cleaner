use std::fmt;

#[derive(Debug)]
pub enum AppError {
    FailedReadingMetadata(rexiv2::Rexiv2Error),
    FailedWritingMetadata(rexiv2::Rexiv2Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::FailedReadingMetadata(err) => write!(f, "Failed reading metadata: {err}"),
            AppError::FailedWritingMetadata(err) => write!(f, "Failed writing metadata: {err}"),
        }
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn error_exit(error: AppError) -> ! {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

