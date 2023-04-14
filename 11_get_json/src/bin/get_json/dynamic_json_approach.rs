use reqwest::blocking::get;
use serde_json::Value;
use std::error::Error;

use crate::shared_constants::URL;

#[cfg(not(test))]
pub fn get_some_json() -> Result<Value, Box<dyn Error>> {
    let response = get(URL)?.text()?;
    let serde_result = serde_json::from_str::<Value>(&response)?;
    Ok(serde_result)
}

#[cfg(test)]
pub fn get_some_json() -> Result<Value, Box<dyn Error>> {
    let input = r#"{"index":0,"name":"AB/CDE/FG/402/test_int4","sts":"on","time":"2021-06-05 03:28:24.044284300 UTC","value":8}"#;
    Ok(serde_json::from_str(input)?)
}
