mod persistent_objects;
mod entities;
mod aggregate;
mod dms_api;

use std::net::SocketAddr;

use dms_api::DmsApi;
use socrates::{error::Error, file_system_storage::FileSystemStorage};

#[tokio::main]
async fn main() -> Result<(), Error>
{
    let storage = FileSystemStorage::new(r"dms\sink".into());
    let address = SocketAddr::from(([127, 0, 0, 1], 7777));
    DmsApi::listen(storage, address).await
}