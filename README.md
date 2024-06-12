# NBP tax liability calculator
Calculator for tax liability from income in foreign currencies. The default implementation uses National Bank of Poland (Narodowy Bank Polski) API for currency exchange rates: https://api.nbp.pl/en.html

```
Usage: tax-calculator --file <FILE> --exchange <EXCHANGE>

Options:
  -f, --file <FILE>          Transactions file path
  -e, --exchange <EXCHANGE>  Exchange that the transactions file is from, options: "binance"
  -h, --help                 Print help
  -V, --version              Print version
```

# CSV file format
## Binance
| Column name | Value |
|-------------|:-------:|
| Date            |   string, format: yyyy/mm/dd    |
|    Type         |   string, "Buy" or "Sell"    |
|    Sent amount         |    float   |
|    Sent Currency        |  string     |
|    Received Amount       |    float   |
|    Received Currency         |   string    |
|    Fee Amount       |   float    |
|    Fee Currency         |    string   |


## Kraken
| Column name | Value |
|-------------|:-------:|
