use serde::{Deserialize, Serialize};

// Login
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub role: String,
    pub name: String,
}