use serde::Deserialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;


trait Pricing {
    fn fetch_price() -> ApiResult;
    fn save_to_file(&self) -> Result<(), String>;
}

#[derive(Debug)]
struct Bitcoin{
    price: f64,
}

#[derive(Debug)]
struct Ethereum{
    price: f64,
}

#[derive(Debug,Deserialize)]
struct BtcEthResponse{
    bitcoin: CoinPrice,
    ethereum:CoinPrice,
}

#[derive(Debug,Deserialize)]
struct CoinPrice {
    usd: f64,
}

// SP500 Section //////////////////////////

#[derive(Debug, Deserialize)]
struct SP500{
    price: f64,
}

#[derive(Debug, Deserialize)]
struct SP500Response{
    chart: Chart,
}

#[derive(Debug, Deserialize)]
struct Chart{
    result: Vec<ChartResult>,
}

#[derive(Debug, Deserialize)]
struct ChartResult{
    indicators: Indicators,
}

#[derive(Debug, Deserialize)]
struct Indicators{
    quote: Vec<Quote>,
}

#[derive(Debug, Deserialize)]
struct Quote{
    high: Vec<Option<f64>>,
}

///////////////////////////////////////////

#[derive(Debug)]
enum ApiResult{
    Success(SP500Response),
    SuccessBtcEth(BtcEthResponse),
    ApiError(String),
    NetworkError(String),
}



/////////////////////////// BITCOIN IMPLEMENTATION ///////////////////////////////////

impl Pricing for Bitcoin {

    fn fetch_price() -> ApiResult {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";

        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<BtcEthResponse>() {
                        Ok(prices) => ApiResult::SuccessBtcEth(prices),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            }
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }


     fn save_to_file(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("bitcoin.txt")
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let line = format!("Bitcoin price: ${:}\n", self.price);
        file.write_all(line.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////



/////////////////////////// ETHEREUM IMPLEMENTATION ///////////////////////////////////

impl Pricing for Ethereum {

   fn fetch_price() -> ApiResult {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";

        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<BtcEthResponse>() {
                        Ok(prices) => ApiResult::SuccessBtcEth(prices),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            }
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("ethereum.txt")
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let line = format!("Ethereum price: ${:}\n", self.price);
        file.write_all(line.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////////


/////////////////////////// SP500 IMPLEMENTATION ///////////////////////////////////


impl Pricing for SP500 {


    fn fetch_price() -> ApiResult {
        let url = "https://query2.finance.yahoo.com/v8/finance/chart/%5EGSPC";
        
        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<SP500Response>() {
                        Ok(sp_price) => ApiResult::Success(sp_price),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            },
            Err(e) => {
                let error_details = format!("Request failed: {}", e);
                ApiResult::NetworkError(error_details)
            },
        }
    }


    fn save_to_file(&self) -> Result<(), String> {
        let price = self.price;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("sp500.txt")
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let line = format!("SP500 price: {:}\n", price);
        file.write_all(line.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        Ok(())
    }
}


////////////////////////////////////////////////////////////////////////////////////////


fn main() -> Result<(), Box<dyn Error>> {

    println!("Prices for BTC, ETH, and SP500 (CTRL + C to stop program)\n");

    loop {
        match Bitcoin::fetch_price() {
            ApiResult::SuccessBtcEth(data) => {
                let btc = Bitcoin { price: data.bitcoin.usd };
                println!("✅ Bitcoin: ${}", btc.price);
                btc.save_to_file().unwrap_or_else(|e| println!("Failed to save Bitcoin price: {}", e));
            }
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
            _ => {}
        }

        match Ethereum::fetch_price() {
            ApiResult::SuccessBtcEth(data) => {
                let eth = Ethereum { price: data.ethereum.usd };
                println!("✅ Ethereum: ${}", eth.price);
                eth.save_to_file().unwrap_or_else(|e| println!("Failed to save Ethereum price: {}", e));
            }
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
            _ => {}
        }

        match SP500::fetch_price() {
            ApiResult::Success(data) => {
                if let Some(result) = data.chart.result.get(0) {
                    if let Some(quote) = result.indicators.quote.get(0) {
                        if let Some(price_option) = quote.high.iter().rev().find_map(|&p| p) {
                            let sp500 = SP500 { price: price_option };
                            println!("✅ SP500: {}", sp500.price);
                            sp500.save_to_file().unwrap_or_else(|e| println!("Failed to save SP500 price: {}", e));
                        } else {
                            println!("❌ SP500 price not found in response");
                        }
                    }
                }
            }
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
            _ => {}
        }

        println!("Waiting 10 seconds\n");
        sleep(Duration::from_secs(10));
    }
}
