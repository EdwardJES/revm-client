mod api;
mod parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::{error, i64};
use text_colorizer::Colorize;
use url::Url;

// TODO:
// impl estimate_gas

#[derive(Serialize, Deserialize)]
struct RPCResponse {
    id: u32,
    jsonrpc: String,
    result: Value,
}

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
                    Ok(rpc_response) => match rpc_response.result {
                        Value::String(val) => {
                            // val[2..] trim 0x
                            println!(
                                "{} result: {:?}",
                                "Blocknumber".green(),
                                i64::from_str_radix(&val[2..], 16)
                            )
                        }
                        _ => (),
                    },
                    Err(err) => eprint!("{}: {:?}", "Blocknumber".red(), err),
                };
            }
            parser::Command::Usage => self.print_usage(),
            parser::Command::UnkownCommand => {
                eprintln!("REVM-Client: recieved unkown command. Please enter a valid command")
            }
        }
    }

    #[tokio::main]
    async fn execute_request(self, json: String) -> Result<RPCResponse, Box<dyn error::Error>> {
        let res = self
            .client
            .post(self.provider_url)
            .header("Content-Type", "application.json")
            .body(json.to_string())
            .send()
            .await?;
        let res_text = res.text().await?;
        println!("{}", res_text);
        let res_json: RPCResponse = from_str(&res_text)?;

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
