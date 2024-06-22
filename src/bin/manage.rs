extern crate sky_rocket_admin as app;

use clap::{Command, Arg, value_parser};

#[tokio::main]
async fn main() {
    let matches = Command::new("manage")
        .about("Manager commands:")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("superuser")
            .about("Superuser management")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("create")
                    .about("Create a new superuser")
                    .arg_required_else_help(true)
                    .arg(Arg::new("username").required(true))
                    .arg(Arg::new("password").required(true))
            )
        )
        .get_matches();
    match matches.subcommand() {
        Some(("superuser", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => app::commands::superuser::create(
            sub_matches.get_one::<String>("username").unwrap().to_owned(),
            sub_matches.get_one::<String>("password").unwrap().to_owned(),
        ).await,
        // Some(("list", _)) => app::commands::list_users().await,
        // Some(("delete", sub_matches)) => app::commands::delete_user(
            // sub_matches.get_one::<i32>("id").unwrap().to_owned(),
        // ).await,
        _ => {},
        }
    }

}