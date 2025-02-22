use crate::settings::settings::get_settings;
use crate::auth::auth::generate_auth;
use crate::req::req::{send_market_req};
use std::env;
use std::path::PathBuf;

mod settings;
mod auth;
mod req;

fn main() {

    let args: Vec<String> = env::args().collect();

    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    let config_path: PathBuf = exe_dir.join("settings.yml");

    let settings = get_settings(config_path.to_str().unwrap());

    let mut symbol_pairs = String::new();

    if args.len() >= 2 {
        if args[1] == "-s" &&args.len() == 3 {
            let symbol = &args[2];
            println!("Target Symbol: {}", symbol);
            let sb = format!("{symbol}-USD");
            symbol_pairs.push_str(&sb);
            // You can now use 'symbol' for further processing
        } else if args[1] == "-d" {
            println!("Use default symbols");
            symbol_pairs.push_str(settings.get("symbol-pairs").expect("'symbol-pairs' not found in settings, ex: symbol-pairs: BTC-USD,ETH-USD"));
        }

    } else {
        eprintln!("Usage: -s <SYMBOL>");
        eprintln!("Usage: -d (display default symbols in settings)");
        return;
    }

    let api_key = settings.get("api_key").expect("API key not found");
    let api_secret = settings.get("api_secret").expect("API secret key not found");

    let domain = "https://api.btse.com/spot";

    let market_api_url = "/api/v3.2/market_summary";
    let funding_rate_api_url = "/api/v2.1/funding_history";


    let market_auth_info = generate_auth(market_api_url, api_key, api_secret, "");
    let _fd_rate_auth_info = generate_auth(funding_rate_api_url, api_key, api_secret, "");


    for symbol in symbol_pairs.split(",") {
        let target_market_api = format!("{domain}{market_api_url}?symbol={symbol}");
        let market_response = send_market_req(&market_auth_info, &target_market_api, "GET", "");

        match market_response {
            Ok(response) => {
                response.display()
            }
            Err(_) => {
                println!("[SPOT] market symbol_pair:{symbol} not found");
            }
        }
    }
}