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

// impl LoginForm{
//     pub fn validate(&self) -> bool {
//         self.email.len() > 0 && self.password.len() > 0
//     }
// }