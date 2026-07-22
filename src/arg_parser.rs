/// Parses the cli arguments given by the user using `clap` crate
use punto::dir_sync;
use clap::{App, Arg, ArgMatches};

/// Parses the args and launchs commands depending on user input
pub fn parse_args_and_launch_commands() {
    let matches = generate_matches();
    call_handlers(matches);
}

/// Generates the matches structure, defining inputs by hand
/// Also metainformation about the cli app is set here
fn generate_matches() -> ArgMatches<'static> {
    let app = App::new("punto -- dotfiles manager")
        .version("0.1")
        .author("Sergio Quijano <sergiquijano@gmail.com>")
        .about("Another dotfiles manager")

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

        // Check dir sync problems
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
                &"download command" => dir_sync::handle_download(yaml_file),
                &"upload command" => dir_sync::handle_upload(yaml_file),
                &"check dir sync problems" => dir_sync::handle_check(yaml_file),
                _ => unreachable!(),
            }
        }
    }
}
