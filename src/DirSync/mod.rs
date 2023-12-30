pub mod directories_descr;
pub mod dir_block;
mod parsers;
use std::path::Path;

use parsers::{YamlDirParser, ParseDirectories};
use anyhow::Context;

use crate::DirSync::parsers::TomlDirParser;

#[derive(Debug)]
enum SupportedFileFormats {
    Yaml,
    Toml,
}

fn get_file_format(file_path: &str) -> anyhow::Result<SupportedFileFormats> {
    let extension = Path::new(file_path)
        .extension()
        .context(format!("Could not get the file extension of the file {}", file_path))?
        .to_str()
        .context("Could not convert file extension object to &str")?;

    let format: SupportedFileFormats = match extension {
        "yaml" => SupportedFileFormats::Yaml,
        "toml" => SupportedFileFormats::Toml,
        _ => return anyhow::bail!(format!("Extension for {} is not supported in our program", file_path)),
    };

    return Ok(format);
}

// TODO -- DESIGN -- the following three functions should return an error?

/// Handle the download command
pub fn handle_download(file_path: &str) {
    println!("📂 Getting files from git repo to your system!");

    // Get the format of the file and parse it depending on the extension
    let format = get_file_format(file_path)
        .context("Could not get file extension!")
        .unwrap();
    let dir_descr = match format {
        SupportedFileFormats::Yaml => YamlDirParser::parse_file(file_path),
        SupportedFileFormats::Toml => TomlDirParser::parse_file(file_path),
    }
        .context("Could not parse file contents to rust object properly :(")
        .unwrap();

    // Download
    dir_descr.download_from_repo_to_system();
}

/// Handle the upload command
pub fn handle_upload(file_path: &str) {
    println!("📂 Uploading files from your system to the repo");

    // Get the format of the file and parse it depending on the extension
    let format = get_file_format(file_path)
        .context("Could not get file extension!")
        .unwrap();
    let dir_descr = match format {
        SupportedFileFormats::Yaml => YamlDirParser::parse_file(file_path),
        SupportedFileFormats::Toml => TomlDirParser::parse_file(file_path),
    }
        .context("Could not parse file contents to rust object properly :(")
        .unwrap();


    // Upload
    dir_descr.upload_from_system_to_repo();
}

pub fn handle_check(file_path: &str) {
    println!("🔎 Checking for problems in your dir syncs");

    // Get the format of the file and parse it depending on the extension
    let format = get_file_format(file_path)
        .context("Could not get file extension!")
        .unwrap();
    let dir_descr = match format {
        SupportedFileFormats::Yaml => YamlDirParser::parse_file(file_path),
        SupportedFileFormats::Toml => TomlDirParser::parse_file(file_path),
    }
        .context("Could not parse file contents to rust object properly :(")
        .unwrap();


    // Check directories specified in the description
    dir_descr.check();
}
