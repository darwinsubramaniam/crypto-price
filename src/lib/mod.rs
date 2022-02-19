use std::fmt::Display;

pub mod fiats;
pub mod cryptos;
mod https_client;
pub mod latest_price;

#[derive(Debug, Copy, Clone)]
pub enum MainTokenFiat {
    SGD = 2808,
    USD = 2781,
    PUNDIX = 9040,
    FUNCTIONX = 3884
}

impl Display for MainTokenFiat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainTokenFiat::SGD => write!(f, "SGD"),
            MainTokenFiat::USD => write!(f, "USD"),
            MainTokenFiat::PUNDIX => write!(f, "PUNDIX"),
            MainTokenFiat::FUNCTIONX => write!(f, "FUNCTIONX"),
        }
    }
}