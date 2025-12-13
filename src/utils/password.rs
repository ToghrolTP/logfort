use bcrypt::{DEFAULT_COST, hash};

use crate::error::AppResult;

pub fn hash_password(password: &str) -> AppResult<String> {
    Ok(hash(password, DEFAULT_COST)?)
}
