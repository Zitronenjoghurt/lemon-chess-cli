use crate::commands::crtusr;
use clap::{Arg, Command};

#[path = "./commands"]
mod commands {
    pub mod crtusr;
}
mod db;

#[tokio::main]
async fn main() {
    let database = db::connect().await.expect("Failed to connect to database.");

    let matches = Command::new("LemonChess-CLI")
        .about("A little CLI tool for doing administrative tasks for the lemon-chess API.")
        .subcommand(
            Command::new("crtusr").about("Creates a new user").arg(
                Arg::new("username")
                    .help("Specifies the username for the new user")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("crtusr", sub_m)) => {
            let username = sub_m.get_one::<String>("username").unwrap();
            if let Err(e) = crtusr::execute(database, username).await {
                println!("Failed to generate user: {}", e);
            }
        }
        _ => println!("Invalid command or missing arguments"),
    }
}
