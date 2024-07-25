use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Rate {
    pub no: String,
    pub effective_date: String,
    pub mid: f64,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeRate {
    pub table: char,
    pub currency: String,
    pub code: String,
    pub rates: Vec<Rate>,
}

#[derive(Debug, Deserialize)]
pub struct Row<'a> {
    pub date: &'a str,
    pub operation: &'a str,
    pub sent_amount: f64,
    pub sent_currency: &'a str,
    pub received_amount: f64,
    pub received_currency: &'a str,
    pub fee_amount: f64,
    pub fee_currency: &'a str,
}
