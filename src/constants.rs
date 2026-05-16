// constants
use std::env;

// debugging mode 
// prints OS, version, and name,
// tests filesystem operations
pub const DEBUG: bool = false;

// program variables
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const OS: &str = env::consts::OS;

// filesystem variables
pub const APPDATA_DIR: &str = "./{NAME}/";
pub const TEST_DIR: &str = "./test/";
pub const INIT_FILE_NAME: &str = "initfile.yml";

// text
pub const INIT_FILE_CONTENT: &str = "ur mom";

