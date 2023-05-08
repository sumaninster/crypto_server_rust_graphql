use serde::Deserialize;
use crate::hitbtc::{get_ticker, get_all_tickers};
use reqwest::Client;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, RootNode,
};

pub struct Context;
impl juniper::Context for Context {}

#[derive(Deserialize, Debug, Clone)]
pub struct Currency {
    pub id: String,
    pub full_name: String,
    pub ask: String,
    pub bid: String,
    pub last: String,
    pub open: String,
    pub low: String,
    pub high: String,
    pub fee_currency: String,
}

// Field resolvers implementation
#[graphql_object(context = Context)]
impl Currency {
    fn id(&self) -> &str {
        &self.id
    }

    fn full_name(&self) -> &str {
        &self.full_name
    }

    fn ask(&self) -> &str {
        &self.ask
    }

    fn bid(&self) -> &str {
        &self.bid
    }

    fn last(&self) -> &str {
        &self.last
    }

    fn open(&self) -> &str {
        &self.open
    }

    fn low(&self) -> &str {
        &self.low
    }

    fn high(&self) -> &str {
        &self.high
    }

    fn fee_currency(&self) -> &str {
        &self.fee_currency
    }
}

async fn fetch_currency_data(symbol: String, full_name: String, fee_currency: String) -> Option<Currency> {
    let client = Client::new();
    match get_ticker(&client, symbol.as_str()).await {
        Ok(ticker) => Some(Currency {
            id: symbol.to_string(),
            full_name: full_name.to_string(),
            ask: ticker.ask,
            bid: ticker.bid,
            last: ticker.last,
            open: ticker.open,
            low: ticker.low,
            high: ticker.high,
            fee_currency: fee_currency.to_string(),
        }),
        Err(_) => None,
    }
}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    async fn currency(symbol: String) -> Option<Currency> {
        match symbol.as_str() {
            "ETHBTC" => fetch_currency_data("ETHBTC".to_string(), "Ethereum".to_string(), "BTC".to_string()).await,
            "BTCUSDT" => fetch_currency_data("BTCUSDT".to_string(), "Bitcoin".to_string(), "USDT".to_string()).await,
            _ => None,
        }
    }

    async fn currencies() -> Vec<Currency> {
        let supported_symbols = ["ETHBTC", "BTCUSDT"];
        let client = Client::new();
        let tickers = get_all_tickers(&client, &supported_symbols).await.unwrap_or_default();

        let mut currencies = Vec::new();
        for (symbol, ticker) in supported_symbols.iter().zip(tickers.into_iter()) {
            let (full_name, fee_currency) = match *symbol {
                "ETHBTC" => ("Ethereum", "BTC"),
                "BTCUSDT" => ("Bitcoin", "USD"),
                _ => continue,
            };

            currencies.push(Currency {
                id: symbol.to_string(),
                full_name: full_name.to_string(),
                ask: ticker.ask,
                bid: ticker.bid,
                last: ticker.last,
                open: ticker.open,
                low: ticker.low,
                high: ticker.high,
                fee_currency: fee_currency.to_string(),
            });
        }

        currencies
    }
}

type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        QueryRoot,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}

