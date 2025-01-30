use crate::{entities::*, persistent_objects::*};
use socrates::error::Error;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};


#[derive(Clone)]
pub struct Aggregate {
    all_folders: HashMap<u32, Arc<RwLock<FolderEntity>>>,
    root_folder: Arc<RwLock<FolderEntity>>,
}

impl Aggregate {
    pub fn default() -> Self {
        let root_folder = Arc::new(RwLock::new(FolderEntity::new(Folder {
            id: 0,
            name: "root".into(),
            parent_id: 0,
        })));
        let mut all_folders = HashMap::new();
        all_folders.insert(0, root_folder.clone());
        Aggregate {
            all_folders,
            root_folder,
        }
    }

    pub fn create_folder(&mut self, folder: Folder) -> Result<(), Error> {
        let id = folder.id;
        let parent_id = folder.parent_id;
        let folder_entity = Arc::new(RwLock::new(FolderEntity::new(folder)));

        self.all_folders.insert(id, folder_entity.clone());

        let parent_folder = self.all_folders.get(&parent_id).ok_or(Error::new( "Parent folder was not found"))?;

        let mut parent_folder = parent_folder.write().expect("Error while locking folder");
        parent_folder.sub_folders.push(folder_entity);

        Ok(())
    }

    pub fn create_file(&mut self, file: File) -> Result<(), Error> {
        let parent_folder = self.all_folders.get(&file.folder_id).ok_or(Error::new( "Parent folder was not found"))?;
        let mut parent_folder = parent_folder.write().expect("Error while locking file");
        parent_folder.files.push(file);

        Ok(())
    }

    pub fn get_folder_tree(&self) -> &Arc<RwLock<FolderEntity>> {
        &self.root_folder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates::json::{from_json, to_json};

    #[test]
    fn test_folder_tree() {
        let mut aggregate = Aggregate::default();

        aggregate
            .create_folder(from_json(r#"{"id":1,"parent_id":0,"name":"folder-1"}"#).unwrap())
            .unwrap();
        aggregate
            .create_folder(from_json(r#"{"id":2,"parent_id":1,"name":"folder-2"}"#).unwrap())
            .unwrap();
        aggregate
            .create_file(from_json(r#"{"id":1,"folder_id":2,"name":"test.txt"}"#).unwrap())
            .unwrap();

        let result = to_json(aggregate.get_folder_tree()).unwrap();

        assert_eq!(
            result,
            r#"{"folder":{"id":0,"parent_id":0,"name":"root"},"sub_folders":[{"folder":{"id":1,"parent_id":0,"name":"folder-1"},"sub_folders":[{"folder":{"id":2,"parent_id":1,"name":"folder-2"},"sub_folders":[],"files":[{"id":1,"folder_id":2,"name":"test.txt"}]}],"files":[]}],"files":[]}"#
        );
    }
}
