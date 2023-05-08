use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Ticker {
    pub ask: String,
    pub bid: String,
    pub last: String,
    pub open: String,
    pub low: String,
    pub high: String,
}

pub async fn get_ticker(client: &Client, symbol: &str) -> Result<Ticker, reqwest::Error> {
    let url = format!("https://api.hitbtc.com/api/3/public/ticker/{}", symbol);
    let response = client.get(&url)
                    .send()
                    .await?;
                    println!("{:?}", response);
    response.json::<Ticker>().await
}

pub async fn get_all_tickers(client: &Client, symbols: &[&str]) -> Result<Vec<Ticker>, reqwest::Error> {
    let joined_symbols = symbols.join(",");
    let url = format!("https://api.hitbtc.com/api/3/public/ticker?symbols={}", joined_symbols);
    let response = client.get(&url)
                    .send()
                    .await?;
    let json_response: Value = response.json().await?;
    let tickers_map: HashMap<String, Ticker> = serde_json::from_value(json_response).unwrap_or_default();

    let tickers: Vec<Ticker> = symbols
        .iter()
        .filter_map(|symbol| tickers_map.get(*symbol).cloned())
        .collect();

    Ok(tickers)
}

