use crate::CommandProcessor;
use crate::YamlProcessor;
use std::process::exit;

#[derive(Debug)]
struct InstallerBlock {
    name: String,
    install_command: String,
    packages: Vec<String>,
}

impl InstallerBlock {
    /// Creates a new InstallerBlock struct
    pub fn new(name: String, install_command: String, packages: Vec<String>) -> Self {
        return InstallerBlock {
            name,
            install_command,
            packages,
        };
    }

    /// Installs all the packages described in the InstallerBlock
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
pub fn handle_install_command() {
    println!("Installing packages");
    let installer_blocks = parse_yaml_installer("./packages.yaml");

    for block in installer_blocks {
        println!("Installing {} block", block.name);
        println!(
            "================================================================================"
        );
        block.install_all_packages();

    }
}

/// Given a installer yaml file, returns a vector with its InstallerBlocks
fn parse_yaml_installer(file_path: &str) -> Vec<InstallerBlock> {
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
        let install_command = value["install_command"].as_str().unwrap().to_string();
        let packages = value["packages"].as_vec().unwrap();
        let packages: Vec<String> = packages
            .into_iter()
            .map(|package| package.as_str().unwrap().to_string())
            .collect();
        installer_blocks.push(InstallerBlock {
            name,
            install_command,
            packages,
        });
    }

    return installer_blocks;
}
