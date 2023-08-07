use std::collections::HashMap;
use serde::Deserialize;
use validator::Validate;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Token {
    pub token: String,    
}

#[derive(Clone, Validate, Default, PartialEq, Deserialize, Debug)]
pub struct LoginForm{
    #[validate(email)]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String
}

impl LoginForm {
    pub fn to_hashmap(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();        
        map.insert("email", self.email.clone());
        map.insert("password", self.password.clone());        
        map
    }
}