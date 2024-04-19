use std::{error::Error, io, process};
use reqwest;
use time;

#[derive(Debug, serde::Deserialize)]
struct Rate {
    no: String,
    effectiveDate: String,
    mid: f64
}

#[derive(Debug, serde::Deserialize)]
struct ExchangeRate {
    table: char,
    currency: String,
    code: String,
    rates: Vec<Rate>
}

#[derive(Debug, serde::Deserialize)]
struct Row<'a> {
    date: &'a str,
    operation: &'a str,
    sent_amount: f64,
    sent_currency: &'a str,
    received_amount: f64,
    received_currency: &'a str,
    fee_amount: f64,
    fee_currency: &'a str,
}

static API_URL: &str = "http://api.nbp.pl/api/exchangerates/rates/a/eur/";

async fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';').from_path("transactions_clean.csv")?;
    for result in rdr.records() {
        let record = result?;
        let row: Row = record.deserialize(None)?;
        println!("{:?}", row);

        let request_url = format!("{API_URL}{0}", row.date);
        println!("{}", request_url);

        let response = reqwest::get(&request_url).await?;
        if response.status().is_success() {
            println!("Success API call!");
        } else {
            let format = format_description!("[year]-[month]-[day]");
            let date = Date::parse(row.date, &format)?;
            println!("{}", date);
        }
        let object = response.json::<ExchangeRate>().await?;
        println!("{:?}", object);
        // println!("{:?}", response);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = example().await {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
