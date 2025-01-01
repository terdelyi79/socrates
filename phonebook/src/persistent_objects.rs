use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Category
{
    pub id: u32,
    pub parent_id: u32,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct Contact
{
    pub id: u32,
    pub category_id: u32,
    pub name: String,
    pub phone_number: String
}