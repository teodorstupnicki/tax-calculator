mod args;
mod exchange_rate;
mod process;
mod models;

use std::error::Error;
use clap::Parser;
use args::Args;
use process::process_binance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    match args.exchange.as_str() {
        "binance" => process_binance(&args.file).await?,
        _ => eprintln!("Unsupported exchange"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
