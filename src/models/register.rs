use std::collections::HashMap;
use serde::Deserialize;
use validator::Validate;

#[derive(Clone, Validate, Default, PartialEq, Deserialize, Debug)]
pub struct RegisterForm{
    #[validate(
        length(min = 1, message = "Primeiro nome é requerido")        
    )]
    pub first_name: String,
    #[validate(
        length(min = 1, message = "Último nome é requerido")        
    )]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(
        length(min = 1, message = "A senha é requerida"),
        length(min = 6, message = "A senha deve ter 6 letras ou mais")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "A senha é requerida"),
        length(min = 6, message = "A senha deve ter 6 letras ou mais")
    )]
    pub confirm_password: String,
}

impl RegisterForm {
    pub fn to_hashmap(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("first_name", self.first_name.clone());
        map.insert("last_name", self.last_name.clone());
        map.insert("email", self.email.clone());
        map.insert("password", self.password.clone());
        map.insert("confirm_password", self.confirm_password.clone());
        map
    }
}