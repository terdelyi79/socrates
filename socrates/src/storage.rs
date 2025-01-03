use std::{future::Future, sync::Arc};
use crate::{error::Error, event::Event};

/// Iterator type for events
type EventIterator = Box<dyn Iterator<Item = Result<(u32, Arc<Event>), Error>>>;

/// Store events in a persistent way.
pub trait Storage : Send + Sync
{
    /// Initialize the storage, and returns an iterator for the events already stored in the storage.
    fn init(&self) -> EventIterator;
    
    /// Add an event to the storage.
    fn add(&mut self, event: Arc<Event>, id: u32) -> impl Future<Output = Result<(), Error>> + Send;   
    
}