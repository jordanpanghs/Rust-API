use serde::{Deserialize, Serialize};

// Login
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}