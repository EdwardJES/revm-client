#[derive(PartialEq)]
#[derive(Debug)]
pub enum Command {
    EstimateGas,
    UnkownCommand
}

pub fn parse_arguments(args: &[String]) -> Command {
    let raw_command = &args[1];
    
    let command = match generate_command(&raw_command) {
        Some(command) => command,
        None => Command::UnkownCommand
    };

    command
}

fn generate_command(arg : &str) -> Option<Command> {
    match arg {
        "estimateGas" => Some(Command::EstimateGas),
        _=> None
    }
}

#[test]
fn test_parse_arguments() {
    assert_eq!(generate_command("estimateGas"), Some(Command::EstimateGas));
    assert_eq!(generate_command("").is_none(), true);
}
