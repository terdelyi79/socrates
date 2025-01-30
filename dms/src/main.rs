mod persistent_objects;
mod entities;
mod aggregate;
mod commands;
mod queries;

use std::sync::Arc;

use aggregate::Aggregate;

use axum::{extract::Path, routing::{get, post}, Extension, Router};
use commands::Commands;
use queries::Queries;
use socrates::{error::Error, event::Event, file_system_storage::FileSystemStorage, sink::Sink};

#[tokio::main]
async fn main() -> Result<(), Error>
{
    // Create a storage to store all data in the file system
    let storage = FileSystemStorage::new(r"dms\sink".into());    
    
    // Create aggregate
    let aggregate = Aggregate::default();
    
    // Create sink
    let  sink = Sink::new(storage, aggregate, &|sink| { Commands::init(sink) } ).await?;    

    let app:Router<()> = Router::new()
        // A generic route for all commands (request body is needed as string before parsing to store as en event)
        .route("/command/{command_name}", post(execute_command))
        // Add specific routes for each query, becase they can have different query parameters
        .route("/query/get_folder_tree", get(Queries::get_folder_tree::<FileSystemStorage>))
        // Add sink as an extension to be able to use by commands and queries
        .layer(Extension(sink));

    // Create a listener to listen on port 9999 of localhost
    let listener = tokio::net::TcpListener::bind("localhost:9999")
        .await
        .unwrap();
    
    // Start to listen and handle requests
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}

async fn execute_command(Extension(mut sink): Extension<Sink<FileSystemStorage, Aggregate>>, Path(command_name): Path<String>, request_body: String) {    
    sink.add(Arc::new(Event::Text(command_name, request_body))).await.unwrap();    
}

