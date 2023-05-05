pub mod directories_descr;
pub mod dir_block;
mod parsers;
use parsers::{YamlDirParser, ParseDirectories};
use anyhow::Context;

/// Handle the download command
pub fn handle_download(yaml_file: &str) {
    println!("📂 Getting files from git repo to your system!");

    // Get directives from yaml file
    let dir_descr = YamlDirParser::parse_file(yaml_file).unwrap();

    // Download
    dir_descr.download_from_repo_to_system();
}

/// Handle the upload command
pub fn handle_upload(yaml_file: &str) {
    println!("📂 Uploading files from your system to the repo");

    // Get directives from yaml file
    let dir_descr = YamlDirParser::parse_file(yaml_file)
        // TODO -- this context is not optimal for the user
        .context("Could not parse contents of Yaml file to rust objects")

        // TODO -- should propagate error or fail here?
        .unwrap();

    // Upload
    dir_descr.upload_from_system_to_repo();
}
