use anyhow::{anyhow, Result};
use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
use std::path::PathBuf;

pub fn load_signer() -> Result<Keypair> {
    // Simplest: read ~/.config/solana/id.json
    let mut p = dirs::home_dir().ok_or_else(|| anyhow!("no home dir"))?;
    p.push(".config/solana/id.json");
    let kp = read_keypair(&p)?;
    Ok(kp)
}

fn read_keypair(path: &PathBuf) -> Result<Keypair> {
    let data = std::fs::read_to_string(path)?;
    let bytes: Vec<u8> = serde_json::from_str(&data)?;
    Keypair::from_bytes(&bytes).map_err(|e| anyhow!(e))
}
