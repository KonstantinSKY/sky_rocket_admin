mod superuser;
use clap::{Command, Arg, ArgMatches};

pub const CMD_NAME: &str = "auth";

#[must_use] 
pub fn commands() -> Command {
    Command::new(CMD_NAME)
        .about("Auth CLI management")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("superuser")
                .about("Superuser management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create New Superuser")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("email").required(true))
                        .arg(Arg::new("password").required(true)),
                ),
        )
}

pub async fn handle(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("superuser", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("create", create_matches)) => {
                    // Call your create function here
                    superuser::create(
                        create_matches.get_one::<String>("username").unwrap().to_owned(),
                        create_matches.get_one::<String>("email").unwrap().to_owned(),
                        create_matches.get_one::<String>("password").unwrap().to_owned(),
                    ).await;
                },
                _ => {
                    eprintln!("Invalid subcommand for superuser");
                }
            }
        },
        _ => {
            eprintln!("Invalid command");
        }
    }
}




