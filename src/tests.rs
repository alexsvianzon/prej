// tests.rs

use super::*;
use crate::filesystem;

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

