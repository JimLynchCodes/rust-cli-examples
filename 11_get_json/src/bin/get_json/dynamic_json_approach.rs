use reqwest::blocking::get;
use serde_json::Value;
use std::error::Error;

use crate::shared_constants::URL;

#[cfg(not(test))]
pub fn get_some_json() -> Result<Value, Box<dyn Error>> {
    let resp = get(URL)?.text()?;
    serde_json::from_str::<Value>(&resp).map_err(Box::<dyn Error>::from)
}

#[cfg(test)]
pub fn get_some_json() -> Result<Value, Box<dyn Error>> {
    let input = r#"{"index":0,"name":"AB/CDE/FG/402/test_int4","sts":"on","time":"2021-06-05 03:28:24.044284300 UTC","value":8}"#;
    serde_json::from_str(input).map_err(Box::<dyn Error>::from)
}
