use std::fmt;

#[derive(Debug)]
pub struct Error
{
    pub message: String
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>
    {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error
{    
}

impl From<std::io::Error> for Error
{
    fn from(value: std::io::Error) -> Self {
        Error{ message: value.to_string() }
    }
}

impl From<serde_json::Error> for Error
{
    fn from(value: serde_json::Error) -> Self {
        Error{ message: value.to_string() }
    }
}