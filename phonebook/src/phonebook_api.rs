use std::net::SocketAddr;

use socrates::{api::Api, error::Error, event::Event, json::{parse_as_json, to_json}, sink::Sink, storage::Storage};

use crate::aggregate::Aggregate;

pub struct PhonebookApi {}

impl PhonebookApi {
    pub async fn run<S>(storage: S) -> Result<(), Error>
    where
        S: Storage + 'static,
    {
        let sink = Sink::new(storage, Aggregate::default());

        let mut api = Api::new(sink);
        api.add_command("CreateCategory", Box::new(Self::create_category))
            .await;
        api.add_command("CreateContact", Box::new(Self::create_contact))
            .await;
        api.add_query(
            "GetCategorizedContacts",
            Box::new(Self::get_categorized_contacts),
        )
        .await;

        api.run(SocketAddr::from(([127, 0, 0, 1], 7777))).await
    }

    // ********************************* Commands ********************************* //

    fn create_category(event: &Event, aggregate: &mut Aggregate) -> Result<(), Error> {
        aggregate.create_category(parse_as_json(event)?)
    }

    fn create_contact(event: &Event, aggregate: &mut Aggregate) -> Result<(), Error> {
        aggregate.create_contact(parse_as_json(event)?)
    }

    // ********************************* Queries ********************************** //

    fn get_categorized_contacts(aggregate: &Aggregate) -> Result<String, Error> {
        to_json(aggregate.get_categorized_contacts())
    }

}