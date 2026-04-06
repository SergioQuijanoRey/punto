mod ArgParser;
mod YamlProcessor;
mod DirSync;

fn main() {
    ArgParser::parse_args_and_launch_commands();
}
