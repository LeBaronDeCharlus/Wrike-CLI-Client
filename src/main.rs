mod args;
mod folders;
mod tasks;

#[macro_use]
extern crate prettytable;
use anyhow::Result;
use args::{Cli, Commands};
use clap::Parser;
use dotenv::dotenv;
use std::env;

fn main() -> Result<()> {
    dotenv().ok();
    let user: String = env::var("WRIKE_USER")?;
    let url: String = env::var("URL")?;
    let token: String = env::var("TOKEN")?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Tasks(name)) => match name.tasks_arg {
            Some(ref _tasks_arg) => {
                println!("No extra argument expected for <tasks>, please see --help");
            }
            None => {
                let path: String = format!(r##"/tasks?responsibles=[{}]"##, user);
                tasks::get_tasks(&url, &path, &token)?;
            }
        },
        Some(Commands::Folders(name)) => match name.folders_arg {
            Some(ref _folders_arg) => {
                println!("No extra argument expected for <tasks>, please see --help");
            }
            None => {
                let path: &str = "/folders";
                folders::get_folders(&url, &path, &token)?;
            }
        },
        None => {}
    }
    //    if &args.list == "folders" {
    //        let path: &str = "/folders";
    //        folders::get_folders(&url, &path, &token)?;
    //    }

    Ok(())
}
