use crate::error::Error;

pub enum Event
{
    // Event with binary content
    Binary(String, Vec<u8>),
    // Event with string content
    Text(String, String)
}

impl Event
{
    pub fn source(&self) -> String
    {
        match self
        {
            Event::Binary(source,_) => source.clone(),
            Event::Text(source,_) => source.clone()
        }
    }

    pub fn text(&self) -> Result<String, Error>
    {
        match self
        {
            Event::Binary(_,_) => Err(Error { message : "Text can't be returned from a binary message.".into() } ),
            Event::Text(_,text) => Ok(text.clone())
        }
    }
}