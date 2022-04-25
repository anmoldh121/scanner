use crate::etherscan;

use std::error::Error;
use std::fs::write;
use etherscan::Etherscan;

pub struct ABI {}

impl ABI {
    pub async fn from_address(address: &str) -> Result<Option<String>, Box<dyn Error>> {
        let response = Etherscan::call(format!("module=contract&action=getabi&address={}", address).as_str()).await?;
        if response.status != "0" {
            write("temp.json", response.result.as_str()).unwrap();
            return Ok(Some(response.result));
        }
        println!("{} {:?}", address, response);
        Ok(None)
    }
}