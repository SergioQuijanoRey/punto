mod CommandProcessor;
mod ArgParser;
mod YamlProcessor;
mod Installer;
mod Downloader;

fn main() {
    let arg_parser = ArgParser::parse_args();
    ArgParser::launch_arg_handlers(arg_parser);
}
