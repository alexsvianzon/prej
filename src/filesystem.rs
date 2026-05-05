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

    pub fn write(content: String) {
        let write_result = fs::write(self.path, content);
        self.content = match write_result {
            Ok(_) => content,
            Err(error) => panic!("Could not write to file because of {error:?}"),
        };
    }
}

#[test]
fn file_struct_test() {
    let content = "This message was generated while testing 'filesystem.rs'";
    let path_str = "./test/unittest.txt";
    let path = Path::new(path_str);
    let mut file: File = File::new(path_str);

    assert_eq!(file.path, path);

    file.write(content.to_string());
    assert_eq!(file::read(), content);
}
