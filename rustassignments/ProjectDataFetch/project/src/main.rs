use serde::Deserialize;
use ureq::serde_json;
use serde_json::json;
use std::fs;
use std::{thread, time::Duration};
// use std::error::Error;

#[derive(Debug)]
struct Bitcoin{
    api_address: String,
    file_name:String,
}

#[derive(Debug)]
struct Ethereum{
    api_address: String,
    file_name:String,
}

#[derive(Debug)]
struct SP500{
    api_address: String,
    file_name:String,
}

pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f32{
        match ureq::get(&self.api_address).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<BTCPriceAPI>() {
                        Ok(data) => {
                            return data.bitcoin.usd as f32;
                        },
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                            0.0
                        },
                    }
                } else {
                    eprintln!("HTTP error: {}", response.status());
                    0.0
                }
            },
            Err(e) => {
                eprintln!("Request failed: {}", e);
                0.0
            },
        }
    }
    
    fn save_to_file(&self){
        // fetch btc price with function
        let btc_price = self.fetch_price();

        if btc_price != 0.0{
            // serialize the price
            let json_content = json!({
                "usd": btc_price,
            });

            // write to file
            fs::write(&self.file_name, json_content.to_string()).expect("Failed to write to file");

            println!("BTC price written to {:?}", self.file_name);
        }
        else {
            println!("Failed to fetch data");
        }
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> f32{
        match ureq::get(&self.api_address).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<ETHPriceAPI>() {
                        Ok(data) => {
                            return data.ethereum.usd as f32;
                        },
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                            0.0
                        },
                    }
                } else {
                    eprintln!("HTTP error: {}", response.status());
                    0.0
                }
            },
            Err(e) => {
                eprintln!("Request failed: {}", e);
                0.0
            },
        }
    }
    
    fn save_to_file(&self){
        // fetch btc price with function
        let eth_price = self.fetch_price();

        if eth_price != 0.0{
            // serialize the price
            let json_content = json!({
                "usd": eth_price,
            });

            // write to file
            fs::write(&self.file_name, json_content.to_string()).expect("Failed to write to file");

            println!("ETH price written to {:?}", self.file_name);
        }
        else {
            println!("Failed to fetch data");
        }

    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f32{
        match ureq::get(&self.api_address).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<SP500PriceAPI>() {
                        Ok(data) => {
                            if let Some(result) = data.chart.result.first(){
                                return result.meta.regularMarketPrice as f32;
                            }
                            
                            else {
                                eprintln!("No data found in the API response");
                                0.0
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                            0.0
                        },
                    }
                } else {
                    eprintln!("HTTP error: {}", response.status());
                    0.0
                }
            },
            Err(e) => {
                eprintln!("Request failed: {}", e);
                0.0
            },
        }
    }
    
    fn save_to_file(&self){
        // fetch btc price with function
        let sp_price = self.fetch_price();

        if sp_price != 0.0{
             // serialize the price
            let json_content = json!({
                "usd": sp_price,
            });

            // write to file
            fs::write(&self.file_name, json_content.to_string()).expect("Failed to write to file");

            println!("S&P 500 price written to {:?}", self.file_name);
        }
        else {
            println!("Failed to fetch data");
        }
       
    }
}

#[derive(Debug, Deserialize)]
struct Cost{
    usd: f32,
}

#[derive(Debug, Deserialize)]
struct BTCPriceAPI {
    bitcoin: Cost,
}
#[derive(Debug, Deserialize)]
struct ETHPriceAPI{
    ethereum: Cost,
}

// Sp500 specific structure (due to different api)
#[derive(Debug, Deserialize)]
struct SP500PriceAPI {
    chart: Chart,
}
// chart contains historical info as well as price, vector needed to store
#[derive(Debug, Deserialize)]
struct Chart {
    result: Vec<Result>,
}
// seperate pricing
#[derive(Debug, Deserialize)]
struct Result {
    meta: Meta,
}
// api uses regularMarketPrice instead of usd
#[derive(Debug, Deserialize)]
struct Meta {
    regularMarketPrice: f32,
}

fn main() {
    // initializing bitcoin api and file
    let btc_api = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string();
    let btc_txt = "btc_prices.json".to_string();
    let b = Bitcoin{api_address:btc_api, file_name:btc_txt};

    // initializing ethereum api and file
    let eth_api = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string();
    let eth_txt = "eth_prices.json".to_string();
    let e = Ethereum{api_address:eth_api, file_name:eth_txt};

    // initializing sp500 api and file
    let sp_api = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d".to_string();
    let sp_txt = "sp_prices.json".to_string();
    let s = SP500{api_address:sp_api, file_name:sp_txt};

    // loop to update pricing every 10 seconds
    // same as while(true)
    loop {
        println!("Fetching and processing prices...");

        println!("BTC: {:?} USD", b.fetch_price());
        println!("ETH: {:?} USD", e.fetch_price());
        println!("SP500: {:?} USD", s.fetch_price());
       
        println!();

        println!("Attempting to write to files...");
        b.save_to_file();
        e.save_to_file();
        s.save_to_file();

        println!();
        thread::sleep(Duration::from_secs(10));
    }
}
