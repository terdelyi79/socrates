use axum::Extension;
use socrates::{json::to_json, sink::Sink, storage::Storage};

use crate::aggregate::Aggregate;

pub struct Queries {}

impl Queries {
    pub async fn get_folder_tree<S: Storage>(
        Extension(sink): Extension<Sink<S, Aggregate>>,
    ) -> String {
        let aggregate = sink.aggregate();
        let aggregate = aggregate.read().await;
        to_json(aggregate.get_folder_tree()).unwrap()
    }    
}
