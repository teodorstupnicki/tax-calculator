use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Transactions file path
    #[arg(short, long)]
    pub file: String,
    
    /// Exchange that the transactions file is from, options: "binance"
    #[clap(short, long)]
    pub exchange: String,
}