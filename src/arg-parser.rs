pub mod arg_parser;

// TODO: validate user input based on command
// e.g. if input == rpc_estimateGas, following args must be valid to this request
enum Command {
    EstimateGas,

}

pub fn parse_arguments(args: &[String]) {
    let raw_command = &args[1];
    
    let command = generate_command(&raw_command);
    
    println("{}", command)
}

fn generate_command(arg : &str) -> Command {
    match arg {
        "estimateGas" => Command::EstimateGas,
        _=> ()
    }
}