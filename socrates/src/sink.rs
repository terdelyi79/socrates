use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{error::Error, event::Event, storage::Storage};

/// Type of the commands can be added to the sink
pub type Command<A> = Box<fn(&Event, &mut A) -> Result<(), Error>>;

/// Receives events and calls registered commands to update the aggregate
pub struct Sink<S: Storage + Send + Sync, A: Send + Sync>
where
    Self: Send + Sync,
{
    aggregate: Arc<RwLock<A>>,
    storage: S,
    handlers: HashMap<String, Command<A>>,
    last_id: u32,
}

impl<S: Storage + Send + Sync, A: Send + Sync> Sink<S, A> {
    /// Create a new sink
    pub fn new(storage: S, aggregate: A) -> Self {
        Self {
            storage,
            aggregate: Arc::new(RwLock::new(aggregate)),
            handlers: HashMap::new(),
            last_id: 0,
        }
    }

    /// Add a command as an event handler for a specific event type
    pub fn add_handler(&mut self, type_id: String, handler: Command<A>) {
        self.handlers.insert(type_id, handler);
    }

    /// Initialize the sink
    pub async fn init(&mut self) -> Result<(), Error> {
        for item in self.storage.init() {
            let (id, event) = item?;
            let mut aggregate = self.aggregate.write().await;
            self.handlers.get(&event.source()).ok_or(Error::new( "Unknown event source."))?(event.as_ref(), &mut aggregate)?;
            self.last_id = id;
        }
        Ok(())
    }

    /// Add an event to the sink
    pub async fn add(&mut self, event: Arc<Event>) -> Result<(), Error> {
        self.last_id += 1;

        // Store the event in the persistent storage
        self.storage.add(event.clone(), self.last_id).await?;

        // Get the handler for the event by type
        let handler = self.handlers.get_mut(&event.source()).unwrap();

        let mut aggregate = self.aggregate.write().await;

        // Call the event type specific handler to handle the event
        handler(event.as_ref(), &mut aggregate)?;

        Ok(())
    }

    /// Get the aggregate
    pub fn aggregate(&self) -> Arc<RwLock<A>> {
        self.aggregate.clone()
    }
}
