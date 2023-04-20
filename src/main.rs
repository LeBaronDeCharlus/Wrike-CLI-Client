#[macro_use]
extern crate prettytable;
use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use prettytable::{Cell, Row, Table};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

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

#[derive(Deserialize, Serialize, Debug)]
struct KindFolder {
    kind: String,
    data: Vec<Folders>,
}

#[derive(Deserialize, Serialize, Debug)]
struct KindTask {
    kind: String,
    data: Vec<Tasks>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Folders {
    title: String,
    id: String,
    #[serde(rename = "childIds")]
    child_ids: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Tasks {
    title: String,
    id: String,
    status: String,
    importance: String,
}

fn get_folders<'a>(url: &'a str, path: &'a str, token: &'a str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let url: String = format!("{}{}", &url, &path);
    let res = client
        .get(url)
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .context("Failed to make get on url")?;

    let folders: KindFolder = res.json::<KindFolder>().context("Could not decode json")?;
    let mut table = Table::new();
    table.add_row(row!["id", "name",]);
    for i in folders.data.iter() {
        if i.child_ids.is_some() {
            table.add_row(Row::new(vec![Cell::new(&i.id), Cell::new(&i.title)]));
        }
    }
    table.printstd();

    Ok(())
}

fn get_tasks<'a>(url: &'a str, path: &'a str, token: &'a str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let url: String = format!("{}{}", &url, &path);
    let res = client
        .get(url)
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .context("Failed to make get on url")?;

    let tasks: KindTask = res.json::<KindTask>().context("Could not decode json")?;
    let mut table = Table::new();
    table.add_row(row!["id", "name", "priority", "status"]);
    for i in tasks.data.iter() {
        table.add_row(Row::new(vec![
            Cell::new(&i.id),
            Cell::new(&i.title),
            Cell::new(&i.importance),
            Cell::new(&i.status),
        ]));
    }

    table.printstd();

    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();
    let args = &Args::parse();

    let user: String = env::var("WRIKE_USER")?;
    let url: String = env::var("URL")?;
    let token: String = env::var("TOKEN")?;

    if &args.list == "folders" {
        let path: &str = "/folders";
        get_folders(&url, &path, &token)?;
    } else if &args.list == "tasks" {
        let path: String = format!(r##"/tasks?responsibles=[{}]"##, user);
        get_tasks(&url, &path, &token)?;
    } else {
        eprintln!("args invalid, please refer to --check");
    }
    Ok(())
}
