use crate::CommandProcessor;
use crate::YamlProcessor;
use std::process::exit;

/// Represents a section of a installer .yaml specification
#[derive(Debug)]
struct InstallerSection {
    name: String,
    install_command: String,
    packages: Vec<String>,
}

impl InstallerSection {
    /// Creates a new InstallerSection struct
    pub fn new(name: String, install_command: String, packages: Vec<String>) -> Self {
        return InstallerSection {
            name,
            install_command,
            packages,
        };
    }

    /// Installs all the packages described in the InstallerSection
    pub fn install_all_packages(&self) {
        for package in &self.packages {
            let command = format!("{} {}", self.install_command, package);
            let command_block = CommandProcessor::CommandBlock::new(
                format!("Install package {}", package),
                false,
                vec![command],
                false,
            );
            command_block.execute();
        }
    }
}

/// Callback for --install cli arg
/// # Arguments
/// - `yaml_file`: file path of the yaml file containing install specification
/// - `section`: section of the specification to install. If it is None, all sections are installed
pub fn handle_install_command(yaml_file: &str, section: Option<&str>) {
    match section{
        None => {
            println!("📦 Installing packages -- all sections");
            install_all_sections(yaml_file);
        }

        Some(section) => {
            println!("📦 Installing packages -- section {}", section);
            install_section(yaml_file, &section);
        }
    }
}

/// Installs all the sections specified in the given yaml file
fn install_all_sections(yaml_file: &str){
    let installer_sections = parse_yaml_installer(yaml_file);
    for section in installer_sections {
        println!("Installing {} block", section.name);
        println!(
            "================================================================================"
        );
        section.install_all_packages();

    }
}

/// Install given section, and only that section
fn install_section(yaml_file: &str, section: &str){
    let installer_sections = parse_yaml_installer(yaml_file);
    for curr_section in installer_sections {
        if curr_section.name == section{
            curr_section.install_all_packages();
        }
    }
}

/// Given a installer yaml file, returns a vector with its InstallerSection
fn parse_yaml_installer(file_path: &str) -> Vec<InstallerSection> {
    let parsed_contents = YamlProcessor::parse_yaml(file_path);
    // TODO -- this block of code is repeated
    let parsed_contents = match parsed_contents{
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Could not parse {}, exiting now", file_path);
            eprintln!("Error code was {}", err);
            exit(-1);
        }
    };

    let mut installer_blocks = vec![];

    for (key, value) in parsed_contents.as_hash().unwrap() {
        let name = key.as_str().unwrap().to_string();
        let install_command = value["install_command"].as_str().expect("Install command cannot be retreived").to_string();

        let empty_packages_vec = vec![];
        let packages = value["packages"].as_vec().unwrap_or(&empty_packages_vec);
        let packages: Vec<String> = packages
            .into_iter()
            .map(|package| package.as_str().unwrap().to_string())
            .collect();
        installer_blocks.push(InstallerSection {
            name,
            install_command,
            packages,
        });
    }

    return installer_blocks;
}
