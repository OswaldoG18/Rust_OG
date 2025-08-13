## Financial Data Fetcher in Rust:

Step 1: Define the 3 main structs (`Bitcoin`, `Ethereum`, and `SP500`)

Step 2: Define the other structs to access the API endpoint data

Step 3: Create a `Pricing` trait and implement it for each struct

Step 4: Implement the Trait for the 3 main structs and call the funnctions (`fetch_price`, `save_to_file`)

Step 5: Match the function on main and display price as well as save the price on the dedicated file


## Features

- **Struct Creation:** Defines distinct Rust structs for Bitcoin, Ethereum, and the S&P 500.
- **Trait Implementation:** Uses a `Pricing` trait to standardize fetching and saving price data.
- **HTTP Requests:** Utilizes the `ureq` crate to interact with public APIs for real-time pricing.
- **Data Parsing:** Employs `serde` to deserialize JSON API responses into Rust structs.
- **Data Storage:** Writes the latest pricing information to individual files per asset.
- **Periodic Execution:** Runs a loop that fetches and saves price data every 10 seconds.
- **Documentation:** The code includes comments for clarity and maintainability.
