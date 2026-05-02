// main.rs

mod commands;
mod constants;

use clap::{Command, arg, command};
use std::fs;
use std::path::Path;
use crate::constants::VERSION;
use crate::constants::NAME;
use crate::constants::OS;

fn main() {
    if constants::DEBUG {
        println!("DEBUG is enabled. Version is {VERSION} of {NAME} app. OS is {OS}.");

        let test_dir_path = Path::new(constants::TEST_DIR);
        let test_dir_display = test_dir_path.display();
        let result = fs::create_dir_all(test_dir_path);
        match result {
            Ok(_) => (),
            Err(error) => panic!("Could not create all directories to path {test_dir_display} because of error: {error:?}"),
        }
        
        let test_file_path = [constants::TEST_DIR, "test.txt"].concat();
        let result = fs::write(&test_file_path, "This file was generated while debugging {NAME}.");
        match result {
            Ok(_) => (),
            Err(error) => panic!("Could not write to the test file at path {test_file_path} because of error: {error:?}"),
        }

        println!("Finished DEBUG! Attempted write to a test file at {test_dir_display}.");
    }

    let matches = command!()
        .subcommand_required(true)
        .version(VERSION)
        .about("A simple registry-based project manager")
        .subcommand(
            Command::new("add")
                .about("Registers a new project")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("go")
                .about("Switch to and initialize a project")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => commands::add(
            sub_matches
        ),
        Some(("go", sub_matches)) => println!(
            "used '{NAME} go', switching to project {}",
            sub_matches
                .get_one::<String>("NAME")
                .expect("Requires a name for 'go'")
        ),
        _ => unreachable!("Command does not exist or was not provided"),
    }
}
