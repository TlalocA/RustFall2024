use serde::Deserialize;
// use std::error::Error;

#[derive(Debug)]

struct Bitcoin{
    api_address: String,
    file_name:String,
}

struct Ethereum{
    api_address: String,
    file_name:String,
}

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
        return 32.0;
    }
    
    fn save_to_file(&self){
        println!("saved to {}", self.file_name);
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> f32{
        return 32.0;
    }
    
    fn save_to_file(&self){
        println!("saved to {}", self.file_name);
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f32{
        return 32.0;
    }
    
    fn save_to_file(&self){
        println!("saved to {}", self.file_name);
    }
}

#[derive(Debug, Deserialize)]
struct Cost{
    usd: i32,
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
    let btc_api = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string();
    let btc_txt = "btc_prices.json".to_string();
    let b = Bitcoin{api_address:btc_api, file_name:btc_txt};
    
    let btc_price = ureq::get(&b.api_address).call().unwrap();

    let b: BTCPriceAPI = btc_price.into_json::<BTCPriceAPI>().unwrap();

    println!("{:?}", b)

    let eth_api = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string();
    let eth_txt = "eth_prices.json".to_string();
    let e = Ethereum{api_address:eth_api, file_name:eth_txt};

    let eth_price = ureq::get(&e.api_address).call().unwrap();

    println!("{:?}", e)

    /*
    let sp_api = "".to_string();
    let sp_txt = "".to_string();
    let s = SP500{api_address:sp_api, file_name:sp_txt};

    let sp_price = ureq::get(&s.api_address).call().unwrap();

    println!("{:?}", s)
    */
}
