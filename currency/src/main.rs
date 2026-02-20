use serde::Deserialize;
use reqwest::blocking::get;
use std::collections::HashMap;

#[derive(Deserialize)]
struct APIResponse{
    result: String,
    base_code: String,
    conversion_rates: HashMap<String, f64>,
}
fn main()-> Result<(), Box<dyn std::error::Error>>{
    let Api ="74055e653fe09718294c6f69";
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/USD", Api);

    let response = get(&url)?;
    let data:APIResponse = response.json()?;

    println!("Base Currency : {}",data.base_code);
    println!("1 USD ={} INR",data.conversion_rates["INR"]);

    Ok(())


}
