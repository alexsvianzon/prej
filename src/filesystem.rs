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
        let byte_content = fs::read(path);
        let string_content = String::from_utf8(byte_content);

        Self {path, string_content};
    }

    pub fn read() {
        let byte_content = fs::read(self.path);
        let string_content = String::from_utf8(byte_content);

        self.content = string_content;
    }
}
