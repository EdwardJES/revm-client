mod api;
mod parser;
use reqwest::Client;
use serde_json::Value;
use text_colorizer::Colorize;
use url::Url;

#[derive(Debug)]
pub struct REVMClient {
    client: Client,
    provider_url: Url,
}

impl REVMClient {
    pub fn new(provider_url: Url) -> Self {
        REVMClient {
            client: Client::new(),
            provider_url,
        }
    }

    pub fn execute(self, args: &[String]) {
        match parser::parse_arguments(&args) {
            Some(args) => Self::execute_command(self, args),
            None => (),
        }
    }

    fn execute_command(self, command: parser::Command) {
        match command {
            parser::Command::EstimateGas => api::ethrpc::estimate_gas(),
            parser::Command::BlockNumber => {
                match self.execute_request(api::ethrpc::block_number()) {
                    Ok(val) => println!("{}:{:?}","Blocknumber".green(), val),
                    Err(err) => eprint!("{:?}", err),
                };
            }
            parser::Command::Usage => self.print_usage(),
            parser::Command::UnkownCommand => {
                eprintln!("REVM-Client: recieved unkown command. Please enter a valid command")
            }
        }
    }

    #[tokio::main]
    async fn execute_request(self, json: String) -> Result<Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post(self.provider_url)
            .header("Content-Type", "application.json")
            .body(json.to_string())
            .send()
            .await?;
        let res_text = res.text().await?;
        let res_json: Value = serde_json::from_str(&res_text.to_string())?;

        Ok(res_json)
    }

    fn print_usage(self) {
        println!(
            "\n{} - execute RPC calls to the EVM JSON interface \n",
            "EVM-Client".green()
        );
        println!("Supported calls:");
        println!("----------------");
        println!("{}: Get the latest block number", "blocknumber".magenta());
        println!("{}: Estimate gas for an eth call", "estimategas".magenta());
    }
}
