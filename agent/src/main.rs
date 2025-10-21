use clap::Parser;

mod jupiter;
mod swap;
mod wallet;

#[derive(Parser, Debug)]
#[command(name = "liquidity-agent")]
struct Args {
    /// input mint (base58)
    #[arg(long)] in_mint: String,
    /// output mint (base58)
    #[arg(long)] out_mint: String,
    /// amount in base units (e.g., 1 USDC = 1_000_000)
    #[arg(long)] amount: u64,
    /// slippage in bps (e.g., 75 = 0.75%)
    #[arg(long, default_value_t = 75)] bps_slippage: u16,
    /// if set, only fetch/print route and min_out
    #[arg(long, default_value_t = false)] dry_run: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let quote = jupiter::best_route(&args.in_mint, &args.out_mint, args.amount).await?;
    let min_out = jupiter::apply_slippage(&quote, args.bps_slippage)?;
    println!(
        "Best route: {} hops; est_out={} (min_out={})",
        quote.route.len(),
        quote.out_amount,
        min_out
    );

    if args.dry_run {
        return Ok(());
    }

    let signer = wallet::load_signer()?;
    let sig = swap::execute(&signer, &quote, min_out).await?;
    println!("Swap tx: {}", sig);
    Ok(())
}
