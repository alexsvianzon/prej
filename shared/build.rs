// build

use std::env;
use std::fs;
use std::path::Path;

use directories::ProjectDirs;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");

    let v = "0.1.0";
    let n = "prej";
    let o = env::consts::OS;

    let _ = fs::create_dir_all(format!("~/Library/Application Support/{}/", n)).unwrap();
    let ad = fs::canonicalize(format!("~/Library/Application Support/{}/", n)).unwrap();
    let ifn = "initfile.yml";

    let ifc = "1s and 0s";

    fs::write(
        &dest_path,
        format!("\
            pub const VERSION: &str = \"{}\";\n\
            pub const NAME: &str = \"{}\";\n\
            pub const OS: &str = \"{}\";\n\
            \n\
            pub const APPDATA_DIR: &str = \"{}\";\n\
            pub const INIT_FILE_NAME: &str = \"{}\";\n\
            \n\
            pub const INIT_FILE_CONTENT: &str = \"{}\";\n",
        v, n, o, ad.display().to_string(), ifn, ifc)
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

