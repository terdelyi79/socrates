use std::{ffi::OsStr, fs::ReadDir, sync::Arc};
use tokio::{fs::OpenOptions, io::{AsyncWriteExt, BufWriter}};
use crate::{error::Error, event::Event, storage::Storage};

const BUFFER_CAPACITY: usize = 1000000;

/// A basic storage implementation of development and demonstrations only.
#[derive(Clone)]
pub struct FileSystemStorage
{
    path: String
}

impl FileSystemStorage
{
    pub fn new(path: String) -> Self
    {
        Self { path }
    }
}

impl Storage for FileSystemStorage
{
    /// Initialize the storage, and returns an iterator for the events already stored in the storage.
    fn init(&self) -> Box<dyn Iterator<Item = Result<(u32, Arc<Event>), Error>>>
    {
        Box::new(FileStorageIterator::new(self.path.clone()))
    }

    /// Add an event to the storage.
    async fn add(&mut self, event: Arc<Event>, id: u32) -> Result<(), Error>
    {
        // Use diffreent extension or text based and binary events.
        let extension = match event.as_ref()
        {
            Event::Binary(_,_) => "bin",
            Event::Text(_,_) => "txt"
        };        

        // Open a new file for the events
        let file_path = format!("{}\\{}_{}.{}", self.path, id, event.source(), extension);
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path).await?;
        
        // Write the content of event to the file
        let mut writer = BufWriter::with_capacity(BUFFER_CAPACITY, file);
        let buf = match event.as_ref()
        {
            Event::Binary(_,content) => content,
            Event::Text(_,content) => content.as_bytes()
        };        
        writer.write_all(buf).await?;
        writer.flush().await?;

        Ok(())
    }
}

pub struct FileStorageIterator
{
    read_dir: ReadDir
}

impl FileStorageIterator
{
    pub fn new(path: String) -> Self
    {        
        Self { read_dir: std::fs::read_dir(path).expect("Error while redaing storage folder.") }
    }
}

impl Iterator for FileStorageIterator {
    type Item = Result<(u32, Arc<Event>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_dir
        .next()
        .map(|file|
            {            
            let path = file?.path();
            let extension = path.extension().and_then(OsStr::to_str).expect("File extension is not a valid UTF-8 string.");
            let file_name_without_extension = path.file_stem().and_then(OsStr::to_str).expect("File name is not a valid UTF-8 string.");
            let i = file_name_without_extension.find('_').expect("File name in the storage should contain a '_' character.");
            let id: String = (&file_name_without_extension[..i]).into();
            let id = id.parse::<u32>().expect("File name should contain a valid integer id.");
            let source: String = (&file_name_without_extension[i + 1..]).into();

            match extension {
                "txt" => {
                    let content = std::fs::read_to_string(path)?;
                    Ok( (id, Arc::new(Event::Text(source, content))))
                },
                "bin" => {
                    let content = std::fs::read(path)?;
                    Ok( (id, Arc::new(Event::Binary(source, content))))
                }
                _ => Err(Error::new( "Invalid file extension"))
            }
            })
    }
}