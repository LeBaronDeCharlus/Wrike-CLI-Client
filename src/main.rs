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
        Some(Commands::Tasks(args)) => {
            let null = String::from("");
            let status = match &args.status {
                Some(status) => status,
                None => "[Active, Completed, Deferred, Cancelled]",
            };
            let search = match &args.search {
                Some(search) => search,
                None => "",
            };
            let folder = match &args.folder {
                Some(folder) => folder,
                None => "",
            };

            let user = if args.me { &user } else { &null };
            if folder != "" {
                let path = format!(
                    r##"/folders/{}/tasks?responsibles=[{}]&title={}&status={}&descendants=true"##,
                    folder, user, search, status
                );
                tasks::get_tasks(&url, &path, &token)?;
            } else {
                let path = format!(
                    r##"/tasks?responsibles=[{}]&title={}&status={}"##,
                    user, search, status
                );
                tasks::get_tasks(&url, &path, &token)?;
            }
        }

        //        Some(Commands::Folders(_folders_arg)) => {}
        None => {}
    }
    Ok(())
}
