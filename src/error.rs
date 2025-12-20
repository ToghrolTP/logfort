use std::fmt;

pub enum AppError {
    Database(sqlx::Error),
    Bcrypt(bcrypt::BcryptError),
    DuplicateEntry(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Bcrypt(e) => write!(f, "Password error: {}", e),
            AppError::DuplicateEntry(field) => write!(f, "Duplicate entry: {}", field),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::Database(db_err) => {
                let msg = db_err.message();
                if msg.contains("UNIQUE constraint failed") {
                    if msg.contains("username") {
                        return AppError::DuplicateEntry("username".to_string());
                    } else if msg.contains("email") {
                        return AppError::DuplicateEntry("email".to_string());
                    }
                }
            },
            _ => {}
        }
        AppError::Database(err)
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::Bcrypt(err)
    }
}

pub type AppResult<T> = Result<T, AppError>;
