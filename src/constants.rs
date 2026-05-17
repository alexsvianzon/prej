// constants.rs

use std::env;

// program variables
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const OS: &str = env::consts::OS;

// filesystem variables
pub const APPDATA_DIR: &str = concat!("./", NAME, "/");
pub const INIT_FILE_NAME: &str = "initfile.yml";

// text
pub const INIT_FILE_CONTENT: &str = "ur mom";

