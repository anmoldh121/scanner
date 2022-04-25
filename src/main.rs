mod scanner;
mod etherscan;
mod abi;
mod helper;

use std::error::Error;
use scanner::Scanner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _scaner = Scanner::new().await?;
    _scaner.watch_transactions().await?;
    Ok(())
}
