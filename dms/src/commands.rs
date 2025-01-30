use socrates::{error::Error, event::Event, json::parse_as_json, sink::Sink, storage::Storage};
use crate::aggregate::Aggregate;

pub struct Commands {    
}

impl Commands {

    pub fn init<S>(sink: &mut Sink<S, Aggregate>) where S: Storage + Send + Sync {
        sink.add_handler("create_folder".into(), Box::new(Commands::create_folder));
        sink.add_handler("create_file".into(), Box::new(Commands::create_file));
    }

    pub fn create_folder(event: &Event, aggregate: &mut Aggregate) -> Result<(), Error> {
        aggregate.create_folder(parse_as_json(event)?)
    }

    pub fn create_file(event: &Event, aggregate: &mut Aggregate) -> Result<(), Error> {
        aggregate.create_file(parse_as_json(event)?)
    }
}
