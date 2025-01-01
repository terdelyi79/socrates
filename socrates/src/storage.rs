use std::{future::Future, sync::Arc};

use crate::{error::Error, event::Event};

type EventIterator = Box<dyn Iterator<Item = Result<(u32, Arc<Event>), Error>>>;

pub trait Storage : Send + Sync
{
    fn init(&self) -> EventIterator;

    #[allow(async_fn_in_trait)]
    fn add(&mut self, event: Arc<Event>, id: u32) -> impl Future<Output = Result<(), Error>> + Send;   
    
}