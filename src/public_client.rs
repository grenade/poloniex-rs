use reqwest;
use errors::*;

use types::{Ticker, CurrencyPair, OrderBook};

use std::collections::HashMap;

type Tickers = HashMap<CurrencyPair, Ticker>;

#[derive(Debug)]
pub struct PublicClient {
    reqwest_client: reqwest::Client
}

impl PublicClient {
    pub fn new() -> Result<Self> {
        let reqwest_client = reqwest::Client::new()?;
        let client = Self { reqwest_client };
        Ok(client)
    }

    pub fn return_ticker(&self) -> Result<Tickers> {
        let tickers =
            self.reqwest_client
            .get("https://poloniex.com/public?command=returnTicker")?
            .send()?
            .json::<Tickers>()?;
        Ok(tickers)
    }

    pub fn return_order_book(&self, currency_pair: CurrencyPair, depth: u32) -> Result<OrderBook> {
        let url = format!("https://poloniex.com/public?command=returnOrderBook&currencyPair={}&depth={}", currency_pair, depth);
        let order_book =
            self.reqwest_client
            .get(&url)?
            .send()?
            .json::<OrderBook>()?;
        Ok(order_book)
    }
}