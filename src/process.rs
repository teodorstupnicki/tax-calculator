use std::error::Error;
use time::Date;
use time::macros::format_description;
use indicatif::ProgressBar;
use csv::ReaderBuilder;
use crate::models::Row;
use crate::exchange_rate::fetch_exchange_rate;

pub async fn process_binance(file: &str) -> Result<(), Box<dyn Error>> {
    let mut buy_net: f64 = 0.0;
    let mut buy_net_pln: f64 = 0.0;
    let mut sell_net: f64 = 0.0;
    let mut sell_net_pln: f64 = 0.0;

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_path(&file)?;
    let total_records = rdr.records().count() as u64;
    rdr = ReaderBuilder::new().delimiter(b';').from_path("transactions_clean.csv")?;
    let progress_bar = ProgressBar::new(total_records);

    for result in rdr.records() {
        let record = result?;
        let row: Row = record.deserialize(None)?;
        let format = format_description!("[year]-[month]-[day]");
        let mut date = Date::parse(row.date, &format)?;
        date = date.previous_day().ok_or("No previous day available")?;

        let currency = match row.operation {
            "Buy" => row.sent_currency.to_lowercase(),
            "Sell" => row.received_currency.to_lowercase(),
            _ => return Err("Unknown operation".into()),
        };

        let exchange_rate = fetch_exchange_rate(date, &currency).await?;

        match row.operation {
            "Buy" => {
                buy_net += row.sent_amount;
                buy_net_pln += row.sent_amount * exchange_rate;
            }
            "Sell" => {
                sell_net += row.received_amount;
                sell_net_pln += row.received_amount * exchange_rate;
            }
            _ => return Err("Unknown operation".into()),
        }
        progress_bar.inc(1);
    }
    progress_bar.finish_and_clear();

    let net_eur = sell_net - buy_net;
    let net_pln = sell_net_pln - buy_net_pln;
    println!("Sum of EUR spent on crypto: {:.2}", buy_net);
    println!("Sum of PLN spent on crypto: {:.2}", buy_net_pln);
    println!("Sum of EUR sold crypto for: {:.2}", sell_net);
    println!("Sum of PLN sold crypto for: {:.2}", sell_net_pln);
    println!("Net amount in EUR: {:.2}", net_eur);
    println!("Net amount in PLN: {:.2}", net_pln);

    Ok(())
}
