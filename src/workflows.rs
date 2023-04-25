extern crate prettytable;
use anyhow::{Context, Result};
use prettytable::{Cell, Row, Table};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
struct KindWorkflow {
    kind: String,
    data: Vec<Workflow>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Workflow {
    id: String,
    name: String,
    #[serde(rename = "customStatuses")]
    custom_statuses: Vec<CustomStatus>,
}

#[derive(Deserialize, Serialize, Debug)]
struct CustomStatus {
    id: String,
    name: String,
}

pub fn get_workflows<'a>(url: &'a str, path: &'a str, token: &'a str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let url: String = format!("{}{}", &url, &path);
    let res = client
        .get(&url)
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .context("Failed to make get workflows")?;
    let workflows: KindWorkflow = res
        .json::<KindWorkflow>()
        .context("Could not decode json")?;
    let mut table = Table::new();
    table.add_row(row!["id", "name", "status id", "status name"]);
    for i in workflows.data.iter() {
        if &i.custom_statuses.len() > &0 {
            for j in i.custom_statuses.iter() {
                table.add_row(Row::new(vec![
                    Cell::new(&i.id),
                    Cell::new(&i.name),
                    Cell::new(&j.id),
                    Cell::new(&j.name),
                ]));
            }
        } else {
            table.add_row(Row::new(vec![
                Cell::new(&i.id),
                Cell::new(&i.name),
                Cell::new(""),
                Cell::new(""),
            ]));
        }
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
        .put(&url)
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
