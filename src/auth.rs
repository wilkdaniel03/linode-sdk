use std::env;

pub struct Token (String);

impl Token {
    pub fn new() -> Self {
        let token = match env::var("LINODE_TOKEN") {
            Ok(t) => t,
            Err(_) => panic!("Failed to find environment variable LINODE_TOKEN")
        };

        Self(token)
    }

    pub fn get_token(&self) -> String {
        self.0.clone()
    }
}

pub struct Api;
