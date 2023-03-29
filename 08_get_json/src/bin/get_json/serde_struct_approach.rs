use reqwest::blocking::get;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

use crate::shared_constants::URL;

#[derive(Debug, Deserialize)]
pub struct User {
    id: u64,
    name: String,
    username: String,
    email: String,
    phone: String,
    website: String,
}

#[cfg(not(test))]
pub fn get_some_json() -> Result<User, Box<dyn std::error::Error>> {
    let response = get(URL)?.text()?;
    let json: Value = serde_json::from_str(&response)?;

    serde_json::from_value(json).map_err(Box::<dyn Error>::from)
}

#[cfg(test)]
pub fn get_some_json() -> Result<User, Box<dyn Error>> {
    Ok(User {
        id: 1,
        name: "name".to_string(),
        username: "username".to_string(),
        email: "email".to_string(),
        phone: "phone".to_string(),
        website: "website".to_string(),
    })
}
