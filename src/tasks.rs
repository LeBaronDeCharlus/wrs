extern crate prettytable;
use anyhow::{Context, Result};
use prettytable::{Cell, Row, Table};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

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
