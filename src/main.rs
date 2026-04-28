use clap::{Command, arg, command};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
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
        Some(("add", sub_matches)) => println!(
            "used '{NAME} add', adding project {}",
            sub_matches
                .get_one::<String>("NAME")
                .expect("Requires a name for 'add'")

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
