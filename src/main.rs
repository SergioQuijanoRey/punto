mod CommandProcessor;
mod ArgParser;

type CallBack = fn() -> ();

fn install_command() {
    println!("Installing packages")
}

fn not_found_command(command: String) {
    println!("Command {} is not valid", command);
    show_help();
}

fn show_help(){
    println!("Punto usage:");
    println!("\tpunto --install: install all packages, defined in install.yml");
    println!("\tpunto --shell: run custom shell scripts, defined in shell.yml");
    println!("\tpunto --all: run all of above commands");
}


fn main() {
    ArgParser::parse_args();
}
