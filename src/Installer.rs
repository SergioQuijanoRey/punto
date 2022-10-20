use crate::CommandProcessor;
use crate::SingleCommand;
use crate::YamlProcessor;
use std::process::exit;

/// Represents a section of a installer .yaml specification
#[derive(Debug)]
struct InstallerSection {

    /// Name of the installer section
    name: String,

    /// Command used to install all packages in this section
    install_command: String,

    /// List of packages to install
    packages: Vec<String>,

    /// Wether or not to use sudo for installing the packages
    /// Some package managers don't use sudo
    sudo: bool,
}

impl InstallerSection {
    /// Creates a new InstallerSection struct
    pub fn new(name: String, install_command: String, packages: Vec<String>, sudo: bool) -> Self {
        return InstallerSection {
            name,
            install_command,
            packages,
            sudo,
        };
    }

    /// Installs all the packages described in the InstallerSection
    /// Returns the packages that failed to install in this section or None if no package failed to
    /// install
    pub fn install_all_packages(&self) -> Option<FailedPackages>{

        // Packages that failed to install
        let mut failed_packages = FailedPackages::new(self.name.clone());

        for package in &self.packages {
            let quiet = false;

            let command_string = format!("{} {}", self.install_command, package);
            let command = SingleCommand::SingleCommand::new(command_string, quiet, self.sudo).expect("Install command failed to build");

            // Run the command. If it fails, add to the list of failed commands
            match command.run(){
                Ok(()) => {},
                Err(_) => {
                    failed_packages.push(package.to_string());
                }
            }
        }

        // No failed packages generated, return None
        if failed_packages.is_empty(){
            return None;
        }

        return Some(failed_packages);
    }
}

#[cfg(test)]
mod test_installer_section{
    use super::InstallerSection;


    #[test]
    pub fn test_failed_packages_are_properly_tracked() -> Result<(), String>{
        // Build a InstallerSection with some non-existing packages
        let sudo = true;
        let section = InstallerSection::new(
            "Install some packages".to_string(),
            "pacman -S --noconfirm".to_string(),
            vec!["git".to_string(), "thispackagedoesnotexist".to_string(), "exa".to_string()],
            sudo,
        );

        // Run the installation and collect the packages
        let failed_packages = section.install_all_packages();

        // There must be at least one failed package
        let failed_packages = match failed_packages{
            None => return Err("Some failed packages were expected".to_string()),
            Some(failed_packages) => failed_packages,
        };

        // Check that the failed package is the one we are expecting
        assert_eq!(failed_packages.failed_packages, vec!["thispackagedoesnotexist".to_string()]);

        // Everything went ok
        return Ok(());
    }

}

/// List of packages that failed to install
/// This packages belong to the same InstallerSection
struct FailedPackages{
    section_name: String,
    failed_packages: Vec<String>,
}

impl FailedPackages{
    /// Creates a new FailedPackages instance
    pub fn new(section_name: String) -> Self{
        return Self{
            section_name: section_name,
            failed_packages: vec![],
        };
    }

    /// Displays to the user all packages that failed to install in a section
    // TODO -- use termion crate for colored output
    pub fn show_failed_packages(&self) {
        eprintln!("==> Some packages in section {} failed to install", self.section_name);
        for package in &self.failed_packages {
            eprintln!("--> {}", package);
        }
    }

    pub fn len(&self) -> usize{
        return self.failed_packages.len();
    }

    pub fn is_empty(&self) -> bool{
        return self.failed_packages.is_empty();
    }

    pub fn push(&mut self, failed_package: String){
        self.failed_packages.push(failed_package);
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
            let failed_packages_per_section = install_all_sections(yaml_file);

            match failed_packages_per_section{
                // Some pacakges failed to install, show them with their section
                Some(failed_packages_per_section) => {
                    println!("Some packages failed to install. Showing them per section");
                    for failed_packages in failed_packages_per_section{
                        failed_packages.show_failed_packages();
                        println!("");
                    }
                }

                // All went good, do nothing
                None => (),
            }
        }

        Some(section) => {
            println!("📦 Installing packages -- section {}", section);
            let failed_packages = install_section(yaml_file, &section);

            match failed_packages{
                // Some packages failed to install, show them
                Some(failed_packages) => {
                    failed_packages.show_failed_packages();
                },

                // All went good, do nothing
                None => (),
            }
        }
    }
}

/// Installs all the sections specified in the given yaml file
/// Returns failed packages per section or None if no package failed to install
fn install_all_sections(yaml_file: &str) -> Option<Vec<FailedPackages>> {

    // Failed packages to install at each installer section
    let mut failed_packages_per_section = vec![];

    let installer_sections = parse_yaml_installer(yaml_file);
    for section in installer_sections {
        println!("Installing {} section", section.name);
        println!(
            "================================================================================"
        );

        // Install all packages
        let failed_packages = section.install_all_packages();

        // Store failed ones if some failed
        match failed_packages{
            // Some package failed, add this FailedPackages to the vector
            Some(failed_packages) => {
                failed_packages_per_section.push(failed_packages);
            },

            // No package failed, do not add to the vector
            None => ()
        }
    }

    // Check if no package failed to install
    if failed_packages_per_section.is_empty(){
        return None;
    }

    return Some(failed_packages_per_section);
}

/// Install given section, and only that section
/// Returns failed packages
/// Returns None if:
///     - No failed packages were generated
///     - No section was found with given name
// TODO -- DESIGN -- two none returns for different situationes, consider using result
// TODO -- DESIGN -- return Result<Option<FailedPackages>, Err>
fn install_section(yaml_file: &str, section: &str) -> Option<FailedPackages>{
    let installer_sections = parse_yaml_installer(yaml_file);
    for curr_section in installer_sections {
        if curr_section.name == section{
            return curr_section.install_all_packages();
        }
    }

    // No section found with this name, no packages installed thus no failed packages generated
    return None;
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
        let sudo = value["sudo"].as_bool().expect("Install command does not have sudo");

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
            sudo,
        });
    }

    return installer_blocks;
}
