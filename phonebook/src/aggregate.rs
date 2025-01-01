use std::{collections::HashMap, sync::{Arc, RwLock}};

use socrates::error::Error;

use crate::{persistent_objects::*, entities::*};

pub struct Aggregate
{
    categories: HashMap<u32, Arc<RwLock<CategoryEntity>>>,
    root_categories: Vec<Arc<RwLock<CategoryEntity>>>
}

impl Aggregate
{
    pub fn default() -> Self
    {
        Aggregate { categories: HashMap::new(), root_categories: Vec::new() }
    }

    pub fn create_category(&mut self, category: Category) -> Result<(), Error>
    {
        let id = category.id;
        let parent_id = category.parent_id;
        let category_entity = Arc::new(RwLock::new(CategoryEntity::new(category)));        
        if parent_id != 0
        {
            let parent_category = self.categories.get(&parent_id).expect("Parent category was not found");

            let mut parent_category = parent_category.write().unwrap();
            parent_category.sub_categories.push(category_entity.clone());
        }
        else
        {
            self.root_categories.push(category_entity.clone());
        }
        self.categories.insert(id, category_entity.clone());

        Ok(())
    }

    pub fn create_contact(&mut self, contact: Contact) -> Result<(), Error>
    {
        let parent_category = self.categories.get(&contact.category_id).expect("Parent category was not found");
        let mut parent_category = parent_category.write().unwrap();
        parent_category.contacts.push(contact);

        Ok(())
    }

    pub fn get_categorized_contacts(&self) -> &Vec<Arc<RwLock<CategoryEntity>>>
    {
        &self.root_categories
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates::json::{from_json, to_json};

    #[test]
    fn test_create_phonebook() {

        let mut aggregate = Aggregate::default();

        aggregate.create_category(from_json(r#"{"id":1,"parent_id":0,"name":"Private"}"#).unwrap()).unwrap();
        aggregate.create_category(from_json(r#"{"id":2,"parent_id":1,"name":"Friends"}"#).unwrap()).unwrap();
        aggregate.create_contact(from_json(r#"{"id":1,"category_id":2,"name":"John Smith","phone_number":"+1-212-456-7890"}"#).unwrap()).unwrap();
        
        let result = to_json(aggregate.get_categorized_contacts()).unwrap();
        
        assert_eq!(
            result,
            r#"[{"category":{"id":1,"parent_id":0,"name":"Private"},"sub_categories":[{"category":{"id":2,"parent_id":1,"name":"Friends"},"sub_categories":[],"contacts":[{"id":1,"category_id":2,"name":"John Smith","phone_number":"+1-212-456-7890"}]}],"contacts":[]}]"#
        );
        
    }
}