use crate::error::Error;

/// Events can be received and stored. It can be either text based or binary
pub enum Event
{
    /// Event with binary content
    Binary(String, Vec<u8>),
    /// Event with string content
    Text(String, String)
}

impl Event
{
    /// Returns the source of the event
    pub fn source(&self) -> String
    {
        match self
        {
            Event::Binary(source,_) => source.clone(),
            Event::Text(source,_) => source.clone()
        }
    }

    /// Return the content of a text based event
    pub fn text(&self) -> Result<String, Error>
    {
        match self
        {
            Event::Binary(_,_) => Err(Error::new( "Text can't be returned from a binary message." )),
            Event::Text(_,text) => Ok(text.clone())
        }
    }
}