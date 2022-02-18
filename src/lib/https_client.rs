use std::collections::HashMap;

use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn build_url(path: &str, query:&Option<(i32,i32)>) -> String {
    let base_url = std::env::var("BASE_URL").unwrap();
    let mut url = format!("{}{}", base_url, path);

    // let mut query = HashMap::new();
    // query.insert("id".to_string(), token_id);
    // query.insert("convert_id".to_string(), fiat_id);

    if let Some(query) = query {
        url.push_str("?");
        url.push_str(&format!("id={}", query.0));
        url.push_str(&format!("&convert_id={}", query.1));
        url
    } else {
        url
    }

}

fn api_key() -> String {
    std::env::var("API_KEY").unwrap()
}

pub async fn get(map_path: String,query:Option<(i32,i32)>) -> Result<Response<Body>> {
    let url = build_url(&map_path,&query);
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("X-CMC_PRO_API_KEY", api_key())
        .body(Body::empty())
        .expect("request builder error");
    dbg!(&req);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    Ok(client.request(req).await?)
}
