use std::net::SocketAddr;

use socrates::{
    api::Api,
    error::Error,
    event::Event,
    json::{parse_as_json, to_json},
    sink::Sink,
    storage::Storage,
};

use crate::aggregate::Aggregate;

pub struct DmsApi {}

impl DmsApi {
    pub async fn run<S>(storage: S) -> Result<(), Error>
    where
        S: Storage + 'static,
    {
        let sink = Sink::new(storage, Aggregate::default());

        let mut api = Api::new(sink);
        api.add_command("CreateCategory", Box::new(Self::create_folder))
            .await;
        api.add_command("CreateContact", Box::new(Self::create_file))
            .await;
        api.add_query("GetCategorizedContacts", Box::new(Self::get_folder_tree))
            .await;

        api.run(SocketAddr::from(([127, 0, 0, 1], 7777))).await
    }

    // ********************************* Commands ********************************* //

    fn create_folder(event: &Event, aggregate: &mut Aggregate) -> Result<(), Error> {
        aggregate.create_folder(parse_as_json(event)?)
    }

    fn create_file(event: &Event, aggregate: &mut Aggregate) -> Result<(), Error> {
        aggregate.create_file(parse_as_json(event)?)
    }

    // ********************************* Queries ********************************** //

    fn get_folder_tree(aggregate: &Aggregate) -> Result<String, Error> {
        to_json(aggregate.get_folder_tree())
    }
}