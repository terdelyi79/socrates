use std::sync::Arc;

use crate::{error::Error, event::Event, storage::Storage};

#[derive(Default)]
pub struct NullStorage
{    
}

impl Storage for NullStorage
{
    fn init(&self) -> Box<dyn Iterator<Item = Result<(u32, Arc<Event>), Error>>>
    {
        Box::new(NullStorageIterator {})
    }

    async fn add(&mut self, _: Arc<Event>, _: u32) -> Result<(), Error>
    {
        Ok(())
    }
}

pub struct NullStorageIterator
{    
}

impl Iterator for NullStorageIterator {
    type Item = Result<(u32, Arc<Event>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

