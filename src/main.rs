use std::{error::Error, io, process};
use reqwest;
use time::Date;
use time::macros::format_description;

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
    let mut buy_net: f64 = 0.0;
    let mut buy_net_pln: f64 = 0.0;
    let mut sell_net: f64 = 0.0;
    let mut sell_net_pln: f64 = 0.0;
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';').from_path("transactions_clean.csv")?;
    for result in rdr.records() {
        let record = result?;
        let row: Row = record.deserialize(None)?;
        println!("{:?}", row);
        
        let format = format_description!("[year]-[month]-[day]");
        let mut date = Date::parse(row.date, &format)?;
        date = date.previous_day().unwrap();
        
        let mut request_url = format!("{API_URL}{0}", date);
        println!("{}", request_url);

        let mut response = reqwest::get(&request_url).await?;
        while response.status().is_client_error() {
            println!("Not a working day.");
            date = date.previous_day().unwrap(); 
            request_url = format!("{API_URL}{0}", date);
            println!("{}", request_url);
            response = reqwest::get(&request_url).await?;
        }
        
        let object = response.json::<ExchangeRate>().await?;
        println!("{:?}", object);
        let rate = object.rates.first().unwrap();
        let exchange_rate = rate.mid;
        // println!("{:?}", response);
        if row.operation == "Buy" {
            buy_net += row.sent_amount;
            let pln_amount = row.sent_amount * exchange_rate;
            buy_net_pln += pln_amount;
        } else if row.operation == "Sell" {
            sell_net += row.received_amount;
            let pln_amount = row.received_amount * exchange_rate;
            sell_net_pln += pln_amount;
        }
    }
    let net = sell_net - buy_net;
    let net_pln = sell_net_pln - buy_net_pln;
    println!("Sum of EUR spent on crypto: {:.2}", buy_net);
    println!("Sum of PLN spent on crypto: {:.2}", buy_net_pln);
    println!("Sum of EUR sold crypto for: {:.2}", sell_net);
    println!("Sum of PLN sold crypto for: {:.2}", sell_net_pln);
    println!("Net amount in EUR: {:.2}", net);
    println!("Net amount in PLN: {:.2}", net_pln);
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = example().await {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
