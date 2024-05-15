use reqwest::Error;
use serde::Deserialize;
use std::io::{self, Write};
use num_format::{Locale, ToFormattedString};

#[derive(Deserialize, Debug)]
struct CoinGeckoPrice {
    usd: f64,
}

#[derive(Deserialize, Debug)]
struct CoinGeckoResponse {
    bitcoin: CoinGeckoPrice,
    ethereum: CoinGeckoPrice,
    cardano: CoinGeckoPrice,
}

async fn fetch_prices() -> Result<CoinGeckoResponse, Error> {
    let api_url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,cardano&vs_currencies=usd";
    let response = reqwest::get(api_url).await?.json::<CoinGeckoResponse>().await?;
    Ok(response)
}

fn prompt_user(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_for_quantity(prompt: &str) -> f64 {
    let input = prompt_user(prompt);
    match input.parse::<f64>() {
        Ok(quantity) => quantity,
        Err(_) => {
            if input.is_empty() {
                0.0
            } else {
                println!("Invalid input, using default value 0.");
                0.0
            }
        }
    }
}

fn format_currency(value: f64) -> String {
    let int_value = value as i64;
    let fraction = (value.fract() * 100.0).round() as i64;
    format!("{}.{:02}", int_value.to_formatted_string(&Locale::en), fraction)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Do you want to see your actual balances or hypothetical balances?");
    println!("1. Actual balances");
    println!("2. Hypothetical balances");

    let choice = prompt_user("Enter your choice (1 or 2): ");

    let (bitcoin_balance, ethereum_balance, cardano_balance) = if choice == "1" {
        let btc_balance = prompt_for_quantity("Enter the quantity of Bitcoin you hold: ");
        let eth_balance = prompt_for_quantity("Enter the quantity of Ethereum you hold: ");
        let ada_balance = prompt_for_quantity("Enter the quantity of Cardano you hold: ");
        (btc_balance, eth_balance, ada_balance)
    } else {
        // Hypothetical balances
        (0.5, 2.0, 1000.0)
    };

    println!("\nFetching USD cryptocurrency prices from CoinGecko...");

    match fetch_prices().await {
        Ok(data) => {
            println!("Bitcoin: ${}", format_currency(data.bitcoin.usd));
            println!("Ethereum: ${}", format_currency(data.ethereum.usd));
            println!("Cardano: ${}", format_currency(data.cardano.usd));

            if choice == "1" {
                println!("\nYour DeFi Portfolio Value in USD:");
                let bitcoin_value = bitcoin_balance * data.bitcoin.usd;
                let ethereum_value = ethereum_balance * data.ethereum.usd;
                let cardano_value = cardano_balance * data.cardano.usd;

                println!("Bitcoin: ${}", format_currency(bitcoin_value));
                println!("Ethereum: ${}", format_currency(ethereum_value));
                println!("Cardano: ${}", format_currency(cardano_value));

                let total_portfolio_value = bitcoin_value + ethereum_value + cardano_value;
                println!("Total Portfolio Value: ${}", format_currency(total_portfolio_value));
            }
        }
        Err(e) => println!("Error fetching prices: {}", e),
    }

    Ok(())
}