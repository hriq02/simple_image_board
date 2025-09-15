use std::{collections::HashMap, fmt, path::PathBuf};
use mime_guess::mime;



pub enum FileType{
    Image,
    Video,
    Gif
}
impl Clone for FileType {
    fn clone(&self) -> Self {
        match self {
            FileType::Image => FileType::Image,
            FileType::Video => FileType::Video,
            FileType::Gif => FileType::Gif,
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::Image => write!(f, "Image"),
            FileType::Video => write!(f, "Video"),
            FileType::Gif => write!(f, "Gif"),
        }
    }
}


impl PartialEq for FileType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileType::Image, FileType::Image) => true,
            (FileType::Video, FileType::Video) => true,
            (FileType::Gif, FileType::Gif) => true,
            _ => false,
        }
    }
}

pub struct FileRegister{
    pub folder : PathBuf,
    pub files : HashMap<u64,FileMetadata> 
}

pub struct FileMetadata{
    pub file_path : String,
    pub file_type : FileType,
    pub extension : String,
    pub file_size : u64
}

impl Clone for FileMetadata{
    fn clone(&self) -> Self {
        FileMetadata{
            file_path : self.file_path.clone(),
            file_type : self.file_type.clone(),
            extension : self.extension.clone(),
            file_size : self.file_size
        }
    }
}

impl fmt::Display for FileMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileMetadata:\nfile_path: {}\nfile_type: {}\nextension: .{}\nfile_size: {} bytes",
               self.file_path, self.file_type, self.extension, self.file_size)
    }
}

impl FileRegister{
    pub fn new(folder_path : &str)->FileRegister{
        FileRegister{
            files : HashMap::new(),
            folder : PathBuf::from(folder_path)
        }
    }

    pub fn get(&self, id : u64) -> Option<FileMetadata>{
        self.files.get(&id).cloned()
    }


    pub fn contains(&self, id : u64) -> bool{
        self.files.contains_key(&id)
    }

    pub fn insert(&mut self, id : u64, file : FileMetadata){
        self.files.insert(id, file.clone());
    }


    pub fn get_path(&self, id : u64) -> Option<String>{
        match self.files.get(&id){
            Some(file) => Some(file.file_path.clone()),
            None => None
        }
    }

    pub fn remove(&mut self, id : &u64){
        self.files.remove(id);
    }

    pub fn map_files(&mut self, files_path : &str) -> Result<(),Box<dyn std::error::Error>>{
        let files = match std::fs::read_dir(files_path){
            Ok(files) => files,
            Err(e) => return Err(Box::new(e))
        };

        for file_r in files{
            let file = match file_r{
                Ok(file) => file,
                Err(e) => return Err(Box::new(e))
            };

            let file_path = file.path();
            
            let mime = mime_guess::from_path(&file_path).first_or_octet_stream();

            let metadata = match file.metadata(){
                Ok(metadata) => metadata,
                Err(e) => return Err(Box::new(e))
            };

            let file_type = match mime.type_(){
                mime::IMAGE => FileType::Image,
                mime::VIDEO => FileType::Video,
                mime::GIF => FileType::Gif,
                _ => FileType::Image
            };
            let file_size : u64 = metadata.len();

            let extension = match file_path.extension(){
                Some(extension) => extension.to_string_lossy().to_string(),
                None => "".to_string()
            };

            let id: u64 = match file.path().file_stem() {
                Some(stem) => {
                    let s = stem.to_string_lossy();
                    s.parse::<u64>()?
                },
                None => return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "File has no stem",
                ))),
            };


            self.insert(id, FileMetadata
                {
                    file_path: file_path.to_string_lossy().to_string(),
                    file_type,
                    extension,
                    file_size
                }
            );
        }
        Ok(())
    }

}


pub fn get_file_type(file_ext : &str) -> FileType{
    match file_ext{
        "jpg" => FileType::Image,
        "jpeg" => FileType::Image,
        "jiff" => FileType::Image,
        "png" => FileType::Image,
        "gif" => FileType::Gif,
        "mp4" => FileType::Video,
        "webm" => FileType::Video,
        "wmv" => FileType::Video,
        _ => FileType::Image
    }
}