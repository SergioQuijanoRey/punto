/// Parses the cli arguments given by the user

use crate::DirSync;
use crate::Commands;
use crate::Installer;
use clap::{App, Arg, ArgMatches};

/// Parses the args and launchs commands depending on user input
pub fn parse_args_and_launch_commands() {
    let matches = generate_matches();
    call_handlers(matches);
}

/// Generates the matches structure, defining inputs by hand
/// Also metainformation about the cli app is set here
fn generate_matches() -> ArgMatches<'static>{
    let app = App::new("punto -- dotfiles manager")
        .version("0.1")
        .author("Sergio Quijano <sergiquijano@gmail.com>")
        .about("Another dotfiles manager")

        // Launch a shell command
        .arg(
            Arg::with_name("shell command")
                .short("-s")
                .long("--shell")
                .value_name("yaml_file")
                .help("Launchs shell commands from yaml file")
                .takes_value(true),
        )

        // Install packages
        .arg(
            Arg::with_name("install command")
                .short("-i")
                .long("--install")
                .value_name("yaml_file")
                .help("Installs packages from yaml file")
        )

        // Download dotfiles from repo to system
        .arg(
            Arg::with_name("download command")
                .short("-d")
                .long("--download")
                .value_name("yaml_file")
                .help("Syncs files and dirs from repo to your system ")
                .takes_value(true),
        )

        // Upload dotfiles from system to repo
        .arg(
            Arg::with_name("upload command")
                .short("-u")
                .long("--upload")
                .value_name("yaml_file")
                .help("Syncs files and dirs from your system to repo")
                .takes_value(true),
        )

        // Specify the section to install
        .arg(
            Arg::with_name("specify install section")
            .long("--section")
            .value_name("section")
            .help(
                "Specify the package section to install (by default all sections of the file are installed) \nCan only be used when using --install"
            )
            .takes_value(true)
            .requires("install command")
        )

        .arg(
            Arg::with_name("check dir sync problems")
            .long("--check")
            .value_name("yaml_file")
            .help(
                "Checks for dir sync problems. Searches for files deleted in a repo (or system) dir that are still present in their system (or repo) dir"
            )
            .takes_value(true)
        );

    let matches = app.get_matches();
    return matches;

}

/// Calls the functions given the cli parameters
fn call_handlers(matches: ArgMatches) {
    for arg in matches.args.iter() {
        let arg_name = arg.0;
        if matches.is_present(arg_name) {
            let yaml_file = matches.value_of(arg_name).unwrap();

            match arg_name {
                &"shell command" => Commands::handle_shell_command(yaml_file),
                &"install command" => {

                    // Check if we passed --section parameter
                    let section = matches.value_of("specify install section");

                    // We launch the installer using this parameter (which can be None)
                    Installer::handle_install_command(yaml_file, section);
                },

                &"download command" => DirSync::handle_download(yaml_file),
                &"upload command" => DirSync::handle_upload(yaml_file),
                &"check dir sync problems" => DirSync::handle_check(yaml_file),
                _ => println!("Command not recognized"),
            }
        }
    }
}
