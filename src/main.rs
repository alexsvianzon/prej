use clap::{Command, arg, command};

fn main() {
    let matches = command!()
        .subcommand_required(true)
        .version("0.1 Alpha")
        .about("A simple registry-based project manager")
        .subcommand(
            Command::new("add")
                .about("Registers a new project")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "used 'pm add', adding project {}",
            sub_matches
                .get_one::<String>("NAME")
                .expect("Requires a name for 'add'")

        ),
        _ => unreachable!("Command does not exist or was not provided"),
    }
}
