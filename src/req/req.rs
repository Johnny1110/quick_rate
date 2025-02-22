use std::collections::HashMap;
use crate::auth::auth::AuthSign;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::req::model::{FundingRate, MarketData};

pub fn send_market_req(
    auth: &AuthSign,
    target_url: &str,
    method: &str,
    body: &str,
) -> Result<MarketData, Box<dyn std::error::Error>> {
    // Create a blocking client
    let client = Client::new();

    // Build the custom headers using the provided auth values
    let mut headers = HeaderMap::new();
    headers.insert("request-api", HeaderValue::from_str(&auth.key)?);
    headers.insert("request-nonce", HeaderValue::from_str(&auth.nonce)?);
    headers.insert("request-sign", HeaderValue::from_str(&auth.sign)?);

    // Build the request based on the method provided
    let request_builder = match method.to_uppercase().as_str() {
        "GET" => client.get(target_url),
        "POST" => client.post(target_url).body(body.to_owned()),
        "PUT" => client.put(target_url).body(body.to_owned()),
        "DELETE" => client.delete(target_url),
        other => return Err(format!("Unsupported HTTP method: {}", other).into()),
    }
        .headers(headers);

    // Send the request and retrieve the response text
    let response = request_builder.send()?;
    let response_text = response.text()?;

    // Deserialize the JSON array into a Vec<MarketData>
    let market_data_vec: Vec<MarketData> = serde_json::from_str(&response_text)?;

    // Return the first element of the array or an error if the array is empty
    market_data_vec
        .into_iter()
        .next()
        .ok_or_else(|| "Empty market data array".into())
}


pub fn _send_fd_rate_req(
    auth: &AuthSign,
    target_url: &str,
    method: &str,
    body: &str,
    symbol: &str, // e.g. "BTC-PERP"
) -> Result<FundingRate, Box<dyn std::error::Error>> {
    // Create a blocking client.
    let client = Client::new();

    // Build the custom headers using the provided auth values.
    let mut headers = HeaderMap::new();
    headers.insert("request-api", HeaderValue::from_str(&auth.key)?);
    headers.insert("request-nonce", HeaderValue::from_str(&auth.nonce)?);
    headers.insert("request-sign", HeaderValue::from_str(&auth.sign)?);

    // Build the request based on the provided HTTP method.
    let request_builder = match method.to_uppercase().as_str() {
        "GET" => client.get(target_url),
        "POST" => client.post(target_url).body(body.to_owned()),
        "PUT" => client.put(target_url).body(body.to_owned()),
        "DELETE" => client.delete(target_url),
        other => return Err(format!("Unsupported HTTP method: {}", other).into()),
    }
        .headers(headers);

    // Send the request and retrieve the response text.
    let response = request_builder.send()?;
    let response_text = response.text()?;

    // Deserialize the response JSON into a HashMap:
    // { "BTC-PERP": [ { ... } ], ... }
    dbg!(&response_text);
    let funding_map: HashMap<String, Vec<FundingRate>> = serde_json::from_str(&response_text)?;
    dbg!(&funding_map);
    // Look up the vector corresponding to the provided symbol.
    let funding_rates = funding_map
        .get(symbol)
        .ok_or_else(|| format!("Symbol {} not found", symbol))?;

    // Return the first element of the array, or an error if the array is empty.
    funding_rates
        .get(0)
        .cloned()
        .ok_or_else(|| "Empty funding rate array".into())
}
