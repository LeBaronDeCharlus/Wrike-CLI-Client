mod args;
mod contacts;
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
            let null = String::new();
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
            if folder.is_empty() {
                let path =
                    format!(r##"/tasks?responsibles=[{user}]&title={search}&status={status}"##,);
                tasks::get_tasks(&url, &path, &token)?;
            } else {
                let path = format!(
                    r##"/folders/{}/tasks?responsibles=[{}]&title={}&status={}&descendants=true"##,
                    &folder, &user, &search, &status
                );
                tasks::get_tasks(&url, &path, &token)?;
            }
        }

        Some(Commands::Folders(args)) => {
            let permalink = match &args.permalink {
                Some(permalink) => permalink,
                None => "",
            };
            let path = format!(r##"/folders?permalink={permalink}"##);
            folders::get_folders(&url, &path, &token)?;
        }

        Some(Commands::Contacts(args)) => {
            let path = format!(
                r##"/contacts?{}"##,
                if args.me {
                    "me=".to_string()
                } else {
                    String::new()
                }
            );
            contacts::get_contacts(&url, &path, &token)?;
        }

        None => {}
    }
    Ok(())
}
