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
    fn save_to_file(&self, price: f32);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f32{
        match ureq::get(&self.api_address).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<BTCPriceAPI>() {
                        Ok(data) => {
                            let price = data.bitcoin.usd as f32;
                            self.save_to_file(price);
                            price
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
    
    fn save_to_file(&self, price: f32){
        // prevent bad price fetch
        if price != 0.0 {
            // serialize the price
            let json_content = json!({
                "usd": price,
            });

            // write to file
            fs::write(&self.file_name, json_content.to_string()).expect("Failed to write to file");

            println!("BTC price written to {:?}", self.file_name);
        }
        else {
            println!("Failed to update BTC price");
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
                            let price = data.ethereum.usd as f32;
                            self.save_to_file(price);
                            price
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
    
    fn save_to_file(&self, price: f32){
        // prevent bad price fetch
        if price != 0.0{
            // serialize the price
            let json_content = json!({
                "usd": price,
            });

            // write to file
            fs::write(&self.file_name, json_content.to_string()).expect("Failed to write to file");

            println!("ETH price written to {:?}", self.file_name);
        }
        else{
            println!("Failed to update ETH price");
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
                                let price = result.meta.regularMarketPrice as f32;
                                self.save_to_file(price);
                                price
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
    
    fn save_to_file(&self, price: f32){
        // prevent bad price fetch
        if price != 0.0{
             // serialize the price
            let json_content = json!({
                "usd": price,
            });

            // write to file
            fs::write(&self.file_name, json_content.to_string()).expect("Failed to write to file");

            println!("S&P 500 price written to {:?}", self.file_name);
        }
        else {
            println!("Failed to update S&P 500 price");
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
     // Create a vector to hold all assets implementing Pricing
     let prices: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin {
            api_address: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string(),
            file_name: "btc_prices.json".to_string(),
        }),
        Box::new(Ethereum {
            api_address: "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string(),
            file_name: "eth_prices.json".to_string(),
        }),
        Box::new(SP500 {
            api_address: "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d".to_string(),
            file_name: "sp_prices.json".to_string(),
        }),
    ];

    
    loop {
        // Iterate through the vector and perform operations
        println!("Fetching and processing prices...");

        for price in &prices {
            let p = price.fetch_price();
            println!("Fetched price: ${}", p);

            //price.save_to_file();
        }

        println!();
     
        thread::sleep(Duration::from_secs(10));
    }
}
