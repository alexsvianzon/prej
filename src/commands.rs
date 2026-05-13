// commands.rs

use crate::constants;
use crate::constants::NAME;

use std::fs;

use rusqlite::Connection;
use ignore::Walk;

pub struct Project {
    id: i32,
    name: String,
    init_file_loc: String,
}

pub fn add(matches: &clap::ArgMatches, conn: Connection) {
    println!("used '{NAME} add', adding project {}",
        matches
            .get_one::<String>("NAME")
            .expect("Requires a name for 'add'"));

    let project = Project {
        id: 0,
        name: matches
            .get_one::<String>("NAME")
            .expect("Requires a name for 'add'")
            .to_string(),
        init_file_loc: String::new(),
    };

    let mut found: bool = false;
    for result in Walk::new("./") {
        let entry = match result {
            Ok(entry) => entry,
            Err(error) => panic!("Got an error while looking for init file: {error}"),
        };

        if entry.file_name() == constants::INIT_FILE_NAME {
            println!("Found the init file!");
            found = true;
            break
        }
    }

    if !found {
        println!("Init file not found, creating one instead");
        let write_res = fs::write(format!("./{}", constants::INIT_FILE_NAME), constants::INIT_FILE_CONTENT);
        match write_res {
            Ok(_) => (),
            Err(error) => panic!("Encountered an error while creating an init file: {error}"),
        }
    }
}

pub fn go(matches: &clap::ArgMatches, conn: Connection) {
    println!("used '{NAME} go', going to project {}",
        matches
            .get_one::<String>("NAME")
            .expect("Requires a name for 'go'"));
}
