use hyper::body::Buf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::https_client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Crypto {
    pub id: i32,
    pub name: String,
    pub symbol: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Status {
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cryptos {
    #[serde(rename = "data")]
    pub cryptos: Vec<Crypto>,
    #[serde(rename = "status")]
    status: Status,
}

impl Cryptos {
    pub async fn new() -> Result<Self> {
        let fiats = Cryptos::fetch_data().await?;
        Ok(fiats)
    }

    pub fn get_pundi_x(&self) -> Option<Crypto> {
        self.cryptos.iter().find(|f| f.id == 9040).cloned()
    }

    pub fn get_funtion_x(&self) -> Option<Crypto> {
        self.cryptos.iter().find(|f| f.id == 3884).cloned()
    }

    pub fn get(&self, id: i32) -> Option<Crypto> {
        self.cryptos.iter().find(|f| f.id == id).cloned()
    }

    async fn fetch_data() -> Result<Self> {
        let fiat_map_path = std::env::var("CRYPTO_MAP_PATH").unwrap();
        let res = https_client::get(fiat_map_path, None).await?;
        let body = hyper::body::aggregate(res).await?;
        Ok(serde_json::from_reader(body.reader())?)
    }
}

#[cfg(test)]
mod test {

    use super::Cryptos;
    use cached::proc_macro::once;

    #[once(sync_writes = true)]
    async fn setup() -> Cryptos {
        dotenv::from_filename("prod.env").ok();
        Cryptos::new().await.unwrap()
    }

    #[tokio::test]
    async fn test_init() {
        let cryptos = setup().await;
        assert!(cryptos.cryptos.len() > 0);
    }

    #[tokio::test]
    async fn test_get_functionx() {
        let cryptos = setup().await;
        let functionx = cryptos.get_funtion_x().unwrap();
        assert_eq!(functionx.id, 3884);
    }

    #[tokio::test]
    async fn test_get_pundix() {
        let cryptos = setup().await;
        let pundix = cryptos.get_pundi_x().unwrap();
        assert_eq!(pundix.id, 9040);
    }
}
