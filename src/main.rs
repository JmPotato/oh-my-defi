mod aptos;

use clap::{Parser, Subcommand};

use crate::aptos::APTOS_COIN_TYPE;

#[derive(Parser, Debug)]
#[command(name = "on-my-finance")]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Spot {
        #[arg(short, long, required = true)]
        cost: f64,

        #[arg(short, long, required = true)]
        amount: f64,

        #[arg(short, long, required = true)]
        price: f64,

        #[arg(short, long, required = true)]
        money: f64,
    },
    #[command(arg_required_else_help = true)]
    LP {
        #[arg(short, long, required = true)]
        source: f64,

        #[arg(short, long, required = true)]
        exchange_rate: f64,

        #[arg(short, long, required = true)]
        fee_rate: f64,

        #[arg(short, long, required = true)]
        lp_rate: f64,
    },
    #[command(arg_required_else_help = true)]
    Aptos {
        #[arg(short, long, required = true)]
        address: String,

        #[arg(short, long, default_value = APTOS_COIN_TYPE)]
        coin_type: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Spot {
            cost,
            amount,
            price,
            money,
        } => {
            let add_amount = money / price;
            let new_amount = amount + add_amount;
            let new_cost = (amount * cost + money) / new_amount;
            let profit = (price - cost) / cost * 100.0;
            let new_profit = (price - new_cost) / new_cost * 100.0;
            println!(
                "[Cost] {} -> {}\n[Amount] {} + {} -> {}\n[Profit] {:.2}% -> {:.2}% \nWith price: {}",
                cost, new_cost, amount, add_amount, new_amount, profit, new_profit, price
            );
        }
        Commands::LP {
            source,
            exchange_rate,
            fee_rate,
            lp_rate,
        } => {
            let source_deposited =
                source / (1.0 / exchange_rate * lp_rate * (1.0 - fee_rate) + 1.0);
            let target_gained = source_deposited / exchange_rate;
            println!(
                "[Source] Deposit: {}\n[Target] Gain: {}\nWith source/target LP rate: {}\nWith source/target exchange rate: {}\nWith fee rate: {}",
                source_deposited, target_gained, lp_rate, exchange_rate, fee_rate
            );
        }
        Commands::Aptos { address, coin_type } => {
            let client = aptos::Client::new();
            let balance = client.get_account_balance(&address, &coin_type).await?;
            println!(
                "{} balance: {} {}",
                address,
                balance,
                coin_type.split("::").last().unwrap()
            );
        }
    }
    Ok(())
}
