mod parser;
mod api;
pub use text_colorizer::Colorize;


#[derive(Debug)]
pub struct Client {
    // URL
    url : url::Url,
}

impl Client {
    pub fn new(url: url::Url) -> Self {
        Client{url}
    }

    pub fn execute(self, args : &[String]) {
        match parser::parse_arguments(&args) {
            Some(args) => Self::parse_command(self, args),
            None => (),
        }
        
    }

    fn parse_command(self, command : parser::Command) {
        match command  {
             parser::Command::EstimateGas => api::ethrpc::estimate_gas(),
             parser::Command::BlockNumber => api::ethrpc::block_number(),
             parser::Command::Usage => self.print_usage(),
             parser::Command::UnkownCommand => {
                eprintln!("REVM-Client: recieved unkown command. Please enter a valid command")
             }
        }
    }

    fn print_usage(self) {
        println!("\n{} - execute RPC calls to the an EVM JSON interface \n", "EVM-Client".green());
        println!("Supported calls");
        println!("{}: Get the latest block number", "blocknumber".magenta());
    }   
}