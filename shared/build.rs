// build

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");

    let v = "0.5.0";
    let n = "prej";
    let o = env::consts::OS;

    let mut ad = PathBuf::new();
    if cfg!(target_os = "windows") {
        let home = std::env::var("USERPROFILE").unwrap();

        let _ = fs::create_dir_all(format!("{home}/AppData/{}/", n)).unwrap();
        ad = fs::canonicalize(format!("{home}/AppData/{}/", n)).unwrap();
    } else if cfg!(target_family = "unix") {
        let home = std::env::var("HOME").unwrap();

        if cfg!(target_os = "macos") {
            let _ = fs::create_dir_all(format!("{home}/Library/Application Support/{}/", n)).unwrap();
            ad = fs::canonicalize(format!("{home}/Library/Application Support/{}/", n)).unwrap();
        } else if cfg!(target_os = "linux") {
            let _ = fs::create_dir_all(format!("{home}/.local/share/{}/", n)).unwrap();
            ad = fs::canonicalize(format!("{home}/.local/share/{}/", n)).unwrap();
        }
    } else {
        unimplemented!();
    }

    let ifn = "Prejfile";
    let ifc = "1s and 0s";

    fs::write(
        &dest_path,
        format!("\
            pub const VERSION: &str = \"{}\";\n\
            pub const NAME: &str = \"{}\";\n\
            pub const OS: &str = \"{}\";\n\
            \n\
            pub const APPDATA_DIR: &str = \"{}/\";\n\
            pub const INIT_FILE_NAME: &str = \"{}\";\n\
            \n\
            pub const INIT_FILE_CONTENT: &str = \"{}\";\n",
        v, n, o, ad.display().to_string(), ifn, ifc)
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

