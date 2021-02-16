use crate::CommandProcessor;
use crate::Installer;
use crate::DirSync;
use std::collections::HashMap;

fn not_found_command(command: String) {
    println!("Command {} is not valid", command);
    show_help();
}

fn show_help() {
    println!("Punto usage:");
    println!("\tpunto --install: install all packages, defined in install.yml");
    println!("\tpunto --shell: run custom shell scripts, defined in shell.yml");
    println!("\tpunto --all: run all of above commands");
    println!("\tpunto --help: shows this help");
}

type CallBack = fn() -> ();
pub struct Handler {
    arg_input: String,
    handler: Box<CallBack>,
}

impl Handler {
    pub fn new(arg_input: String, handler: CallBack) -> Self {
        return Handler {
            arg_input,
            handler: Box::new(handler),
        };
    }

    /// Runs the callback for the Handler
    pub fn launch_callback(&self) {
        (self.handler)();
    }
}

pub struct ArgParser {
    handlers: Vec<Handler>,
}

impl ArgParser {
    pub fn new() -> Self {
        return ArgParser { handlers: vec![] };
    }

    pub fn add_handler(&mut self, handler: Handler) {
        self.handlers.push(handler);
    }

    /// Finds a handler by its arg_input
    pub fn find_handler(&self, arg_input: String) -> Option<&Handler> {
        for handler in &self.handlers {
            if handler.arg_input == arg_input {
                return Some(handler);
            }
        }

        return None;
    }
}

fn show_version(){
    println!("punto v0.0.1 -- Still in development");
}

/// Gets my arg parser, defined by hand
pub fn parse_args() -> ArgParser {
    // Arguments and their callbacks

    let mut arg_parser = ArgParser::new();
    arg_parser.add_handler(Handler::new(
        "--install".to_string(),
        Installer::handle_install_command,
    ));
    arg_parser.add_handler(Handler::new(
        "--shell".to_string(),
        CommandProcessor::handle_shell_command,
    ));
    arg_parser.add_handler(Handler::new("--download".to_string(), DirSync::handle_download));
    arg_parser.add_handler(Handler::new("--upload".to_string(), DirSync::handle_upload));
    arg_parser.add_handler(Handler::new("--help".to_string(), show_help));
    arg_parser.add_handler(Handler::new("--version".to_string(), show_version));
    arg_parser.add_handler(Handler::new("-v".to_string(), show_version));

    return arg_parser;
}

/// Launchs the callbacks for given args by cli command
pub fn launch_arg_handlers(arg_parser: ArgParser) {
    // Iterate over given arguments
    let args = std::env::args();
    for (indx, arg) in args.enumerate() {
        // First arg is the cli app name
        if indx == 0 {
            continue;
        }

        let command = arg_parser.find_handler(arg.clone());
        match command {
            Some(command) => command.launch_callback(),
            None => not_found_command(arg),
        };
    }
}
