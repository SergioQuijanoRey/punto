use crate::CommandProcessor;
use crate::DirSync;
use crate::Installer;
use clap::{App, Arg, ArgMatches};

// TODO -- better naming
pub fn new_arg_parser() {
    let matches = App::new("punto -- dotfiles manager")
        .version("0.1")
        .author("Sergio Quijano <sergiquijano@gmail.com>")
        .about("Another dotfiles manager")
        .arg(
            Arg::with_name("shell command")
                .short("-s")
                .long("--shell")
                .value_name("yaml_file")
                .help("Launchs shell commands from yaml file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("install command")
                .short("-i")
                .long("--install")
                .value_name("yaml_file")
                .help("Installs packages from yaml file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("download command")
                .short("-d")
                .long("--download")
                .value_name("yaml_file")
                .help("Syncs files and dirs from repo to your system ")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("upload command")
                .short("-u")
                .long("--upload")
                .value_name("yaml_file")
                .help("Syncs files and dirs from your system to repo")
                .takes_value(true),
        )
        .get_matches();


    call_handlers(matches);
}

fn call_handlers(matches: ArgMatches){
    for arg_name in vec!["shell command", "install command", "download command", "upload command"]{
        if matches.is_present(arg_name){
            let yaml_file = matches.value_of(arg_name).unwrap();

            match arg_name{
                "shell command" => CommandProcessor::handle_shell_command(yaml_file),
                "install command" => Installer::handle_install_command(yaml_file),
                "download command" => DirSync::handle_download(yaml_file),
                "upload command" => DirSync::handle_upload(yaml_file),
                _ => println!("Command not recognized")
            }
        }
    }
}
