use serde::Deserialize;
//use std::fs::File;
//use std::io::{Write, BufReader, BufRead};
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
        
        //let price = self.fetch_price();
        //let content = format!("{{\"bitcoin\": {}}}\n", price);
        //std::fs::write(&self.file_name, content).expect("Unable to write to file");
        
        println!("Saved Bitcoin price to {}", self.file_name);
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
        println!("saved to {}", self.file_name);
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f32{
        match ureq::get(&self.api_address).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<SP500PriceAPI>() {
                        Ok(data) => {
                            return data.sp500.usd as f32;
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
        println!("saved to {}", self.file_name);
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
#[derive(Debug, Deserialize)]
struct SP500PriceAPI{
    sp500: Cost,
}

fn main() {
    // initializing bitcoin api and file
    let btc_api = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string();
    let btc_txt = "btc_prices.json".to_string();
    let b = Bitcoin{api_address:btc_api, file_name:btc_txt};
    
    //let btc_price = ureq::get(&b.api_address).call().unwrap();
    //let b: BTCPriceAPI = btc_price.into_json::<BTCPriceAPI>().unwrap();

    // initializing ethereum api and file
    let eth_api = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string();
    let eth_txt = "eth_prices.json".to_string();
    let e = Ethereum{api_address:eth_api, file_name:eth_txt};

    //let eth_price = ureq::get(&e.api_address).call().unwrap();
    //let e: ETHPriceAPI = eth_price.into_json::<ETHPriceAPI>().unwrap();

    // initializing sp500 api and file
    let sp_api = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d".to_string();
    let sp_txt = "sp_prices.json".to_string();
    let s = SP500{api_address:sp_api, file_name:sp_txt};

    //let sp_price = ureq::get(&s.api_address).call().unwrap();

    // loop to update pricing every 10 seconds
    // same as while(true)
    loop {
        println!("Fetching and processing prices...");

        println!("BTC: {:?} USD", b.fetch_price());
        println!("ETH: {:?} USD", e.fetch_price());
        println!("SP500: {:?} USD", s.fetch_price());

        thread::sleep(Duration::from_secs(10));
    }
}
