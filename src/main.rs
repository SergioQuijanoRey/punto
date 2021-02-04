struct CommandArgumentParsing{
    command: String,
    action: Box<dyn Fn() -> ()>,
}


/// Reads the shell yaml config file and executes commands
fn shell_command(){
    println!("Running shell commands");
}

fn install_command(){
    println!("Installing packages")
}
fn main() {
    let args = std::env::args();
    let mut arg_parser = vec![];
    arg_parser.push(CommandArgumentParsing{
        command: "--shell".to_string(),
        action: Box::new(shell_command),
    });

    arg_parser.push(CommandArgumentParsing{
        command: "--install".to_string(),
        action: Box::new(install_command),
    });

    for arg in args{
        for current_arg_parser in &arg_parser{
            if arg == current_arg_parser.command{
               (&current_arg_parser.action)();
            }
        }
    }
}
