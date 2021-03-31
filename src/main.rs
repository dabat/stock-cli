use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let api_key = "c1idnh748v6vit20t3k0";
    let args: Vec<String> = env::args().collect();
    let mut symbols: Vec<String> = args
        .clone()
        .drain(1..)
        .map(|symbol| symbol.to_uppercase())
        .collect();
    if symbols.len() == 0 {
        symbols.push("AAPL".to_string());
    }
    // println!("Hello, args! => {:?} ({} elements)", args, args.len());
    // println!("symbols:{:?}", symbols);

    println!("Current prices:");
    let mut results: Vec<CompanyQuote> = vec![];
    for symbol in symbols {
        let result = CompanyQuote::get(&symbol, api_key).await?;
        results.push(result.clone());
        println!("{} ${}", &result.symbol.unwrap_or_default(), &result.c);
        // println!("{} {}", symbol, result.c);
    }
    println!("full results:");
    for result in results {
        println!("{:?}", result);
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CompanyQuote {
    symbol: Option<String>,
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: i128,
}

impl CompanyQuote {
    async fn get(symbol: &String, api_key: &str) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );
        let url = Url::parse(&*url)?;
        let mut result = reqwest::get(url).await?.json::<CompanyQuote>().await?;
        result.symbol = Some(symbol.clone());
        Ok(result)
    }
}
