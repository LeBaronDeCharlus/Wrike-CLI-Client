extern crate prettytable;

use anyhow::{Context, Result};
use prettytable::{Cell, Row, Table};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct KindContact {
    kind: String,
    data: Vec<Contact>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Contact {
    id: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    me: Option<bool>,
}

pub fn get_contacts<'a>(url: &'a str, path: &'a str, token: &'a str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let url: String = format!("{}{}", &url, &path);
    let res = client
        .get(url)
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .context("Failed to make get on url")?;

    let contacts: KindContact = res.json::<KindContact>().context("Could not decode json")?;
    let mut table = Table::new();
    table.add_row(row!["id", "first_name", "last_name", "me"]);
    for i in contacts.data.iter() {
        table.add_row(Row::new(vec![
            Cell::new(&i.id),
            Cell::new(&i.first_name),
            Cell::new(&i.last_name),
            Cell::new(&i.me.map_or(String::from(""), |b| b.to_string())),
        ]));
    }
    table.printstd();
    Ok(())
}
