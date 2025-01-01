use std::{ops::{Deref, DerefMut}, sync::{Arc, RwLock}};

use serde::{Deserialize, Serialize};

use crate::persistent_objects::{Category, Contact};

#[derive(Serialize, Deserialize)]
pub struct CategoryEntity
{
    pub category: Category,
    pub sub_categories: Vec<Arc<RwLock<CategoryEntity>>>,
    pub contacts: Vec<Contact>
}

impl CategoryEntity
{
    pub fn new(category: Category) -> Self
    {
        CategoryEntity { category, sub_categories: Vec::new(), contacts: Vec::new() }
    }
}

impl Deref for CategoryEntity {
    type Target = Category;

    fn deref(&self) -> &Self::Target {
        &self.category
    }
}

impl DerefMut for CategoryEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.category
    }
}