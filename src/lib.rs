mod parser;
mod api;

#[derive(Debug)]
pub struct Client {
    // URL
    url : String,
}

impl Client {
    pub fn new(url: String) -> Self {
        Client{url}
    }

    pub fn execute(self, args : &[String]) {
        Self::parse_command(self, parser::parse_arguments(&args));
    }

    fn parse_command(self, command : parser::Command) {
        match command  {
             parser::Command::EstimateGas => api::ethrpc::estimate_gas(),
             parser::Command::BlockNumber => api::ethrpc::block_number(),
             parser::Command::UnkownCommand => {
                println!("REVM-Client: recieved unkown command. Please enter a valid command")
             }
        }
    }   
}