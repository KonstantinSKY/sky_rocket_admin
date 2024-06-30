extern crate sky_rocket_admin as project;

use clap::Command;

use project::auth::cmd as auth;

#[tokio::main]
async fn main() {
    let matches = Command::new("run")
        .about("Manager of CLI Projects commands:")
        .arg_required_else_help(true)
        // add command from Applications here
        .subcommand(auth::commands())

        .get_matches();


        match matches.subcommand() {
            Some((auth::CMD_NAME, arg_matches)) => auth::handle(arg_matches).await,
            _ => println!("No command Found"),
        }
    }

