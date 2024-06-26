use std::{error::Error, process};
use reqwest;
use time::Date;
use time::macros::format_description;
use indicatif::ProgressBar;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Transactions file path
    #[arg(short, long)]
    file: String,
    
    /// Exchange that the transactions file is from, options: "binance"
    #[clap(short, long)]
    exchange: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Rate {
    no: String,
    effective_date: String,
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

enum Exchange {
    Binance,
    Kraken,
}

enum Operation {
    Buy,
    Sell,
}

static API_URL: &str = "http://api.nbp.pl/api/exchangerates/rates/a";

async fn process_binance(file: &str) -> Result<(), Box<dyn Error>> {
    let mut buy_net: f64 = 0.0;
    let mut buy_net_pln: f64 = 0.0;
    let mut sell_net: f64 = 0.0;
    let mut sell_net_pln: f64 = 0.0;
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';').from_path(&file)?;
    let count = rdr.records().count() as u64;
    rdr = csv::ReaderBuilder::new()
        .delimiter(b';').from_path("transactions_clean.csv")?;
    let pb = ProgressBar::new(count);
    for result in rdr.records() {
        let record = result?;
        let row: Row = record.deserialize(None)?;
        // println!("{:?}", row);
        
        let format = format_description!("[year]-[month]-[day]");
        let mut date = Date::parse(row.date, &format)?;
        date = date.previous_day().unwrap();
        let mut currency_lowercase: String = String::new(); 
        if row.operation == "Buy" {
            currency_lowercase = row.sent_currency.to_lowercase();
        }
        else if row.operation == "Sell" {
            currency_lowercase = row.received_currency.to_lowercase();
        }
        let mut request_url = format!("{API_URL}/{1}/{0}", date, currency_lowercase.as_str());
        // println!("{}", request_url);

        let mut response = reqwest::get(&request_url).await?;
        while response.status().is_client_error() {
            // println!("Not a working day.");
            date = date.previous_day().unwrap(); 
            request_url = format!("{API_URL}/{1}/{0}", date, currency_lowercase);
            // println!("{}", request_url);
            response = reqwest::get(&request_url).await?;
        }
        
        let object = response.json::<ExchangeRate>().await?;
        // println!("{:?}", object);
        let rate = object.rates.first().unwrap();
        let exchange_rate = rate.mid;
        if row.operation == "Buy" {
            buy_net += row.sent_amount;
            let pln_amount = row.sent_amount * exchange_rate;
            buy_net_pln += pln_amount;
        } else if row.operation == "Sell" {
            sell_net += row.received_amount;
            let pln_amount = row.received_amount * exchange_rate;
            sell_net_pln += pln_amount;
        }
        pb.inc(1);
    }
    pb.finish_and_clear();
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
    let args = Args::parse();
    if &args.exchange == "binance" {
        if let Err(err) = process_binance(&args.file).await {
            println!("Error while processing Binance transactions file: {}", err);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
