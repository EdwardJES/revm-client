use text_colorizer::Colorize;

#[derive(PartialEq, Debug)]
pub enum Command {
    EstimateGas,
    BlockNumber,
    Usage,
    UnkownCommand,
}

pub fn parse_arguments(args: &[String]) -> Option<Command> {
    let command = generate_command(args);

    command
}

fn generate_command(args: &[String]) -> Option<Command> {
    match args[0].as_str() {
        "estimategas" => {
            if args.len() < 2 {
                eprintln!(
                    "{}: estimategas requires two arguments: <{}>, <{}> ",
                    "ERROR".bright_red(),
                    "command".blue(),
                    "eth_call".blue()
                );
                None
            } else {
                return Some(Command::EstimateGas);
            }
        }
        "blocknumber" => Some(Command::BlockNumber),
        "usage" => Some(Command::Usage),
        _ => Some(Command::UnkownCommand),
    }
}

#[test]
fn test_parse_arguments() {
    // String is Vec<T>, ptr, len, cap (for modifying)
    // &str is T  ptr, len (viewing)
    assert_eq!(
        generate_command(&["estimategas".to_string(), "eth_call".to_string()]),
        Some(Command::EstimateGas)
    );
    assert_eq!(generate_command(&["estimategas".to_string()]), None);
    assert_eq!(
        generate_command(&["blocknumber".to_string()]),
        Some(Command::BlockNumber)
    );
    assert_eq!(
        generate_command(&["".to_string()]),
        Some(Command::UnkownCommand)
    );
}
