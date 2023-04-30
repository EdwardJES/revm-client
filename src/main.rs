use dotenv::dotenv;
use revm_client::Client;
use std::env;
pub use url::Url;

fn main() {
    // Instantiate client with RPC url from env
    dotenv().ok();
    let raw_url = env::var("PROVIDER_URL").expect("ERROR: provider URL not found");
    let provider_url = Url::parse(&raw_url).expect("Failed to parse provider URL");
    let client = Client::new(provider_url);
    println!("{:#?}", client);

    // Parse CLI arguments
    let args: Vec<String> = env::args().skip(1).collect();
    client.execute(&args);
    // Return resp to client
}
