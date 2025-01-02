use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Folder
{
    pub id: u32,
    pub parent_id: u32,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct File
{
    pub id: u32,
    pub folder_id: u32,
    pub name: String    
}