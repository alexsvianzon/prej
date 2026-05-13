// commands.rs
use crate::constants::NAME;
use rusqlite::{Connection, Result};

pub fn add(matches: &clap::ArgMatches, conn: Connection) {
    println!("used '{NAME} add', adding project {}",
        matches
            .get_one::<String>("NAME")
            .expect("Requires a name for 'add'"));
}

pub fn go(matches: &clap::ArgMatches, conn: Connection) {
    println!("used '{NAME} go', going to project {}",
        matches
            .get_one::<String>("NAME")
            .expect("Requires a name for 'go'"));
}
