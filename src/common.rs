use std::path::PathBuf;
pub struct File {
    pub file_name: String,
    pub path: PathBuf,
}

impl File {
   fn from(path: PathBuf) -> Option<Self>{
       let file_name = match path.file_name(){
           Some(f) => f,
           None => return None,
       };
       
       let file_name = match file_name.to_str(){
           Some(f) => f.to_string(),
           None => return None
       };

       Some(File{file_name, path})

   } 
}