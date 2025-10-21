# 💧 Solana Liquidity Agent
Safe, scriptable liquidity ops on Solana (quotes via Jupiter, min-out guard, optional DCA).

## Quickstart
````bash
git clone https://github.com/florawizard/solana-liquidity-agent
cd solana-liquidity-agent/agent
cargo run -- --in-mint <USDC_MINT> --out-mint <TOKEN_MINT> --amount 1000000 --bps-slippage 75 --dry-run
````

## Status
- Quotes + min-out working.
- Tx submission stubbed for safety; wire Jupiter `/swap` when ready.
