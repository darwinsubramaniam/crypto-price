use std::{collections::HashMap, hash::Hash};

use hyper::body::Buf;
use serde::Deserialize;
use serde_json::Value;

use super::https_client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    #[serde(rename = "data")]
    data: HashMap<String, Value>,
    #[serde(rename = "status")]
    status: HashMap<String, Value>,
}

impl Price {
    pub async fn get_quotes(token_id: i32, fiat_id: i32) -> Result<f64> {
        let price_map_path = std::env::var("PRICE_MAP_PATH").unwrap();

        //?id=1&convert_id=2781
        let res = https_client::get(price_map_path, Some((token_id, fiat_id))).await?;
        let body = hyper::body::aggregate(res).await?;
        let price: Price = serde_json::from_reader(body.reader())?;

        let price = price
            .data
            .get(&token_id.to_string())
            .expect("token_id not found")
            .get("quote".to_string())
            .expect("quote not found")
            .get(fiat_id.to_string())
            .expect("fiat_id not found")
            .get("price")
            .expect("price not found")
            .as_f64()
            .expect("price could not be parsed to f64");
        Ok(price)
    }
}

#[cfg(test)]
mod test {
    use crate::lib::MainTokenFiat;

    use super::Price;

    async fn setup() {
        dotenv::from_filename("prod.env").ok();
    }

    #[tokio::test]
    async fn test_pundi_x_to_usd() {
        setup().await;
        let price = Price::get_quotes(MainTokenFiat::PUNDIX as i32, MainTokenFiat::USD as i32)
            .await
            .unwrap();
        dbg!(&price);
        assert_eq!(price > 0.0, true);
    }

    #[tokio::test]
    async fn test_function_x_to_usd() {
        setup().await;
        let price = Price::get_quotes(MainTokenFiat::FUNCTIONX as i32, MainTokenFiat::USD as i32)
            .await
            .unwrap();
        dbg!(&price);
        assert_eq!(price > 0.0, true);
    }

    #[tokio::test]
    async fn test_pundi_x_to_sgd() {
        setup().await;
        let price = Price::get_quotes(MainTokenFiat::PUNDIX as i32, MainTokenFiat::SGD as i32)
            .await
            .unwrap();
        dbg!(&price);
        assert_eq!(price > 0.0, true);
    }

    #[tokio::test]
    async fn test_function_x_to_sgd() {
        setup().await;
        let price = Price::get_quotes(MainTokenFiat::FUNCTIONX as i32, MainTokenFiat::SGD as i32)
            .await
            .unwrap();
        dbg!(&price);
        assert_eq!(price > 0.0, true);
    }
}
