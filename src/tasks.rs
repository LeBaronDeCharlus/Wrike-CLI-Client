extern crate prettytable;
use anyhow::{Context, Result};
use prettytable::{Cell, Row, Table};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
struct KindTask {
    kind: String,
    data: Vec<Tasks>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Tasks {
    title: String,
    id: String,
    status: String,
    importance: String,
    permalink: String,
}

pub fn get_tasks<'a>(url: &'a str, path: &'a str, token: &'a str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let url: String = format!("{}{}", &url, &path);
    let res = client
        .get(url)
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .context("Failed to make get tasks")?;
    let tasks: KindTask = res.json::<KindTask>().context("Could not decode json")?;
    let mut table = Table::new();
    table.add_row(row!["id", "name", "priority", "url", "status"]);
    for i in &tasks.data {
        table.add_row(Row::new(vec![
            Cell::new(&i.id),
            Cell::new(&i.title),
            Cell::new(&i.importance),
            Cell::new(&i.permalink),
            Cell::new(&i.status),
        ]));
    }

    table.printstd();

    Ok(())
}

#[warn(dead_code)]
pub fn _put_tasks<'a>(
    url: &'a &str,
    path: &'a str,
    id: &'a str,
    status: &'a str,
    token: &'a str,
) -> Result<()> {
    let mut map = HashMap::new();
    map.insert("status", &status);
    let client = reqwest::blocking::Client::new();
    let url: String = format!("{}{}", &url, &path);
    let _res = client
        .put(url)
        .json(&map)
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .context("Failed to update task")?;

    let mut table = Table::new();
    table.add_row(row!["id", "status"]);
    table.add_row(row![&id, &status]);

    Ok(())
}
