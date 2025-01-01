mod persistent_objects;
mod entities;
mod aggregate;
mod phonebook_api;

use phonebook_api::PhonebookApi;
use socrates::{error::Error, file_storage::FileStorage};

#[tokio::main]
async fn main() -> Result<(), Error>
{
    let storage = FileStorage::new(r"phonebook\sink".into());    
    PhonebookApi::run(storage).await
}