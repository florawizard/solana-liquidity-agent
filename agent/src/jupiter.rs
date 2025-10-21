use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RouteStep { pub label: String }

#[derive(Deserialize, Debug)]
pub struct Quote {
    pub route: Vec<RouteStep>,
    #[serde(rename="outAmount")] pub out_amount: u64,
    pub tx: Option<String>, // placeholder for built tx
}

pub async fn best_route(in_mint: &str, out_mint: &str, amount: u64) -> Result<Quote> {
    let url = format!(
        "https://quote-api.jup.ag/v6/quote?inputMint={}&outputMint={}&amount={}",
        in_mint, out_mint, amount
    );
    let resp = reqwest::get(&url).await?.error_for_status()?;
    let v: serde_json::Value = resp.json().await?;
    // simplified extractor for demo; robust code would parse a struct
    let route = v.get("routePlan").and_then(|x| x.as_array()).unwrap_or(&vec![])
        .iter().map(|_| RouteStep { label: "hop".into() }).collect();
    let out_amount = v.get("outAmount").and_then(|x| x.as_str()).unwrap_or("0").parse::<u64>().unwrap_or(0);
    Ok(Quote { route, out_amount, tx: None })
}

pub fn apply_slippage(q: &Quote, bps: u16) -> Result<u64> {
    let mul = 10_000u64.saturating_sub(bps as u64);
    Ok(q.out_amount.saturating_mul(mul) / 10_000)
}
