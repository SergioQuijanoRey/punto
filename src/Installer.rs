use crate::CommandProcessor;
use crate::YamlProcessor;

#[derive(Debug)]
struct InstallerBlock {
    name: String,
    install_command: String,
    packages: Vec<String>,
}

// Callback for cli arg
pub fn install_command() {
    println!("Installing packages");
    let installer_blocks = parse_yaml_installer("./packages.yaml");

    for block in installer_blocks {
        println!("Installing {} block", block.name);
        println!(
            "================================================================================"
        );

        for package in block.packages {
            install_package(&block.install_command, &package);
        }
    }
}

fn install_package(install_command: &str, package: &str) {
    let command = format!("{} {}", install_command, package);
    let command_block = CommandProcessor::CommandBlock::new(
        format!("Install package {}", package),
        false,
        vec![command],
        false,
    );
    command_block.execute();
}

// Given a installer yaml file, returns a vector with its InstallerBlocks
fn parse_yaml_installer(file_path: &str) -> Vec<InstallerBlock> {
    let parsed_contents = YamlProcessor::parse_yaml(file_path);
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
