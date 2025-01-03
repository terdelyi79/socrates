use serde::{de::DeserializeOwned, Serialize};

use crate::{error::Error, event::Event};

/// Parse the content of an event as JSON to a specific type
pub fn parse_as_json<T>(event: &Event) -> Result<T, Error> where T: DeserializeOwned
{
    Ok(serde_json::from_str(&event.text()?)?)
}

/// Serialize any object to JSON
pub fn to_json<T>(object: &T) -> Result<String, Error> where T: Serialize
{
    Ok(serde_json::to_string(object)?)
}

/// Deserialize an object from JSON
pub fn from_json<T>(s: &str) -> Result<T, Error> where T: DeserializeOwned
{
    Ok(serde_json::from_str(s)?)
}