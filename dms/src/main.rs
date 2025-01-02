mod persistent_objects;
mod entities;
mod aggregate;
mod dms_api;

use dms_api::DmsApi;
use socrates::{error::Error, file_storage::FileStorage};

#[tokio::main]
async fn main() -> Result<(), Error>
{
    let storage = FileStorage::new(r"phonebook\sink".into());    
    DmsApi::run(storage).await
}