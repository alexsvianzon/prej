// filesystem.rs

use std::fs;
use std::path::Path;

pub struct File {
    path: Path,
    content: String,
}

impl File {
    pub fn new(str_path: &str) -> Self {
        let path = Path::new(str_path);
        
        let read_result = fs::read(path);
        let byte_content = match read_result {
            Ok(content) => content,
            Err(error) => panic!("Could not read from file, error: {error:?}."),
        };

        let conv_result = String::from_utf8(byte_content);
        let string_content = match conv_result {
            Ok(content) => content,
            Err(error) => panic!("Could not convert utf8 to string, error: {error:?}."),
        };

        Self {path, string_content};
    }

    pub fn read() {
        let read_result = fs::read(self.path);
        let byte_content = match read_result {
            Ok(content) => content,
            Err(error) => panic!("Could not read from file, error: {error:?}."),
        };

        let conv_result = String::from_utf8(byte_content);
        let string_content = match conv_result {
            Ok(content) => content,
            Err(error) => panic!("Could not convert utf8 to string, error: {error:?}."),
        };

        self.content = string_content;
    }

    pub fn write(content: String) {
        let write_result = fs::write(self.path, content);
        self.content = match write_result {
            Ok(_) => content,
            Err(error) => panic!("Could not write to file because of {error:?}"),
        };
    }
}

