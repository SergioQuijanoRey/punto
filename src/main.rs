mod arg_parser;
mod dir_sync;
mod yaml_processor;

fn main() {
    arg_parser::parse_args_and_launch_commands();
}
