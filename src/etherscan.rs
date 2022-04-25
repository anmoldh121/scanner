use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use ethers::types::H256;
use ethers::types::Address;

const API_KEY: &str = "ZGTQK71XEKB7Z251P5V4NXWDWRJDTEZF5K";
const BASE_URL: &str = "https://api.etherscan.io/api?";

pub struct Etherscan {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub status: String,
    pub message: String,
    pub result: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EventResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<Event>
}

impl Etherscan {
    pub async fn call(url: &str) -> Result<Response, Box<dyn Error>> {
        let body = reqwest::get(format!("{}{}&apikey={}", BASE_URL, url, API_KEY)).await?.text().await?;
        let response: Response = serde_json::from_str(body.as_str()).unwrap();
        Ok(response)
    }

    pub async fn event(url: &str) -> Result<EventResponse, Box<dyn Error>> {
        let body = reqwest::get(format!("{}{}&apikey={}", BASE_URL, url, API_KEY)).await?.text().await?;
        let response: EventResponse = serde_json::from_str(body.as_str()).unwrap();
        Ok(response)
    }
}
