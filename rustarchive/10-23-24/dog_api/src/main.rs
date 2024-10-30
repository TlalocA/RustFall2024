use ureq;

use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn main() {
    let url = "https://dog.ceo/api/breeds/image/random";

    let req = ureq::get(url).call().unwrap();
    let content = req.into_json::<DogImage>();

    println!("{:?}", content);
}
