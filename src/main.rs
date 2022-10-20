mod CommandProcessor;
mod ArgParser;
mod YamlProcessor;
mod Installer;
mod DirSync;
mod SingleCommand;

fn main() {
    ArgParser::parse_args_and_launch_commands();
}
