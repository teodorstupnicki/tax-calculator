use std::{error::Error, io, process};

struct Rate {
    no: String,
    effectiveDate: String,
    mid: f64
}

struct ExchangeRate {
    table: char,
    currency: String,
    code: String,
    rates: Vec<Rate>
}

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("transactions.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
    println!("Hello, world!");
}
