use dotenv::dotenv;
use std::env;

fn main() {
    // Instantiate client with RPC url from env
    dotenv().ok();

    let provider_url = env::var("PROVIDER_URL").expect("ERROR: provider URL not found");

    println!("Provider URL: {}", provider_url);
    // Parse CLI arguments

    // Execute API req based on CLI arguments
    // Return resp to client
}
