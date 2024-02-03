use bevy::prelude::*;
use chrono::{prelude::*, Duration, Local};
use serde_json::Value;
use std::env;

async fn fetch_aggs_ticker_range(
    stocks_ticker: &String,
    multiplier: i32,
    timespan: &str,
    from: &String,
    to: &String,
) -> Result<Value, reqwest::Error> {
    let api_key = env::var("POLYGON_KEY").expect("MISSING POLYGON KEY!");
    let url = format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/range/{}/{}/{}/{}?apiKey={}",
        stocks_ticker, multiplier, timespan, from, to, api_key
    );

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(serde_json::json!(body))
}

fn bevy(resp_json: &Value) {
    println!("{}", resp_json);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing ticker");
        std::process::exit(1);
    }

    let ticker = &args[1];

    let date_format = "%Y-%m-%d";
    let local: DateTime<Local> = Local::now();
    let one_year_ago: DateTime<Local> = local - Duration::days(365);
    let formatted_date = local.format(date_format).to_string();
    let formatted_date_year_ago = one_year_ago.format(date_format).to_string();

    match fetch_aggs_ticker_range(ticker, 1, "day", &formatted_date_year_ago, &formatted_date).await
    {
        Ok(_result) => bevy(&_result),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
