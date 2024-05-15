# DeFi Portfolio Tracker

This Rust application helps you track the value of your DeFi portfolio in USD. It fetches real-time prices for Bitcoin, Ethereum, and Cardano from the CoinGecko API and calculates the total portfolio value based on your holdings.

## Features

- Fetches real-time cryptocurrency prices from CoinGecko.
- Allows you to enter actual balances or use hypothetical balances.
- Calculates and displays the value of your portfolio in USD.
- Formats currency values with commas for better readability.

## Prerequisites

- Rust and Cargo installed on your system.
- An internet connection to fetch data from the CoinGecko API.

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/defiPortfolioTracker.git
    cd defiPortfolioTracker
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

## Usage

1. Run the application:

    ```sh
    cargo run --release
    ```

2. Choose between actual balances and hypothetical balances:

    ```plaintext
    Do you want to see your actual balances or hypothetical balances?
    1. Actual balances
    2. Hypothetical balances
    Enter your choice (1 or 2):
    ```

3. If you choose actual balances (option 1), enter the quantities of Bitcoin, Ethereum, and Cardano you hold:

    ```plaintext
    Enter the quantity of Bitcoin you hold:
    Enter the quantity of Ethereum you hold:
    Enter the quantity of Cardano you hold:
    ```

4. The application will fetch the latest prices and display the value of your portfolio:

    ```plaintext
    Fetching USD cryptocurrency prices from CoinGecko...
    Bitcoin: $XX,XXX.XX
    Ethereum: $X,XXX.XX
    Cardano: $X.XX

    Your DeFi Portfolio Value in USD:
    Bitcoin: $XX,XXX.XX
    Ethereum: $X,XXX.XX
    Cardano: $X.XX
    Total Portfolio Value: $XX,XXX.XX
    ```

## Error Handling

- If there is an error fetching prices from CoinGecko, an error message will be displayed.

## Dependencies

- `reqwest` for making HTTP requests.
- `serde` for deserializing JSON responses.
- `num-format` for formatting numbers with commas.

## License

This project is licensed under the MIT License.

## Acknowledgements

- [CoinGecko API](https://www.coingecko.com/en/api) for providing cryptocurrency price data.

Feel free to contribute to this project by submitting issues or pull requests. Your feedback and contributions are welcome!