#[macro_use]
extern crate prettytable;
use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::env;

mod folders;
mod tasks;

#[derive(Parser, Debug)]
#[command(name = "wr")]
#[command(author = "Le Baron de Charlus <f00b4rch@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "access Wripe through CLI", long_about = None)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List : project, tasks
    #[arg(short, long, value_name = "Items")]
    list: String,
}

fn main() -> Result<()> {
    dotenv().ok();
    let args = &Args::parse();

    let user: String = env::var("WRIKE_USER")?;
    let url: String = env::var("URL")?;
    let token: String = env::var("TOKEN")?;

    if &args.list == "folders" {
        let path: &str = "/folders";
        folders::get_folders(&url, &path, &token)?;
    } else if &args.list == "tasks" {
        let path: String = format!(r##"/tasks?responsibles=[{}]"##, user);
        tasks::get_tasks(&url, &path, &token)?;
    } else {
        eprintln!("args invalid, please refer to --check");
    }
    Ok(())
}
