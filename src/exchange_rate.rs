use reqwest;
use time::Date;
use crate::models::ExchangeRate;
use std::error::Error;

static API_URL: &str = "http://api.nbp.pl/api/exchangerates/rates/a";

pub async fn fetch_exchange_rate(date: Date, currency: &str) -> Result<f64, Box<dyn Error>> {
    let mut current_date = date;
    loop {
        let request_url = format!("{API_URL}/{currency}/{current_date}");
        let response = reqwest::get(&request_url).await?;
        if response.status().is_success() {
            let exchange_rate: ExchangeRate = response.json().await?;
            if let Some(rate) = exchange_rate.rates.first() {
                return Ok(rate.mid);
            }
        }
        current_date = current_date.previous_day().ok_or("No previous day available")?;
    }
}
