use std::collections::HashMap;
use hyper::{body::Buf, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{https_client, MainTokenFiat};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Deserialize,Clone)]
pub struct Fiat {
    pub id: i32,
    pub name: String,
    pub symbol: String,
}


#[derive(Debug,Deserialize,Clone)]
pub struct Fiats{
    #[serde(rename="data")]
    pub fiats: Vec<Fiat>,
}

impl Fiats {
    pub async fn new() -> Result<Self> {
        let fiats = Fiats::fetch_data().await?;
        Ok(fiats)
    }

    pub fn get_usd(&self) -> Option<Fiat> {
        self.fiats.iter().find(|f| f.id == MainTokenFiat::USD as i32).cloned()
    }

    pub fn get_sgd(&self) -> Option<Fiat> {
        self.fiats.iter().find(|f| f.id == MainTokenFiat::SGD as i32).cloned()
    }

    pub fn get(&self, symbol: &str) -> Option<Fiat> {
        self.fiats.iter().find(|f| f.symbol == symbol).cloned()
    }

    async fn fetch_data() -> Result<Self> {
        let fiat_map_path = std::env::var("FIAT_MAP_PATH").unwrap();
        let res = https_client::get(fiat_map_path,None).await?;
        let body = hyper::body::aggregate(res).await?;
        Ok(serde_json::from_reader(body.reader())?)
    }
}

#[cfg(test)]
mod test {

    use super::Fiats;
    use cached::proc_macro::once;

    #[once(sync_writes = true)]
    async fn setup() -> Fiats {
        dotenv::from_filename("sandbox.env").ok();
        Fiats::new().await.unwrap()
    }

    #[tokio::test]
    async fn test_init() {
        let fiats = setup().await;
        assert_eq!(fiats.fiats.len() > 0, true);
    }

    #[tokio::test]
    async fn test_get_usd() {
        let usd = setup().await.get_usd().unwrap();
        assert_eq!(usd.id, 2781);
    }

    #[tokio::test]
    async fn test_get_sgd() {
        let sgd = setup().await.get_sgd().unwrap();
        assert_eq!(sgd.id, 2808);
    }
}
