use dotenv::dotenv;
use revm_client::Client;
use std::env;
use url::Url;

fn main() {
    // Instantiate client with RPC url from env
    dotenv().ok();

    let raw_url = env::var("PROVIDER_URL").expect("ERROR: provider URL not found");
    let provider_url = Url::parse(&raw_url).expect("Failed to parse provider URL");
    let client = Client::new(provider_url.as_str().to_string());

    println!("{:#?}", client);
    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();

    client.execute(&args);
    // Execute API req based on CLI arguments
    // Return resp to client
}
