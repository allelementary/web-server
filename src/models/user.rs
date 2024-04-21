use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub login: String,
    pub email: String,
    pub password: String,
    pub wallet_address: Option<String>,
}

impl User {
    pub fn new(
        login: String,
        email: String,
        password: String,
        wallet_address: Option<String>,
    ) -> Self {
        User {
            id: None,
            login,
            email,
            password,
            wallet_address,
        }
    }
}
