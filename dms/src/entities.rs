use std::{ops::{Deref, DerefMut}, sync::{Arc, RwLock}};

use serde::{Deserialize, Serialize};

use crate::persistent_objects::{Folder, File};

#[derive(Serialize, Deserialize)]
pub struct FolderEntity
{
    pub folder: Folder,
    pub sub_folders: Vec<Arc<RwLock<FolderEntity>>>,
    pub files: Vec<File>
}

impl FolderEntity
{
    pub fn new(folder: Folder) -> Self
    {
        FolderEntity { folder, sub_folders: Vec::new(), files: Vec::new() }
    }
}

impl Deref for FolderEntity {
    type Target = Folder;

    fn deref(&self) -> &Self::Target {
        &self.folder
    }
}

impl DerefMut for FolderEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.folder
    }
}