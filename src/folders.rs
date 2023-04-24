extern crate prettytable;
use anyhow::{Context, Result};
use prettytable::{Cell, Row, Table};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct KindFolder {
    kind: String,
    data: Vec<Folders>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Folders {
    title: String,
    id: String,
    #[serde(rename = "childIds")]
    child_ids: Option<Vec<String>>,
}

pub fn _get_folders<'a>(url: &'a str, path: &'a str, token: &'a str) -> Result<()> {
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
