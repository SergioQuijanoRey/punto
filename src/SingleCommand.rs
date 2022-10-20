use std::process::Command;

/// Represents a shell command to execute
#[derive(Debug)]
pub struct SingleCommand{

    /// Wether or not output the result of the command
    quiet: bool,

    /// The comand itself
    command: String,

    /// If we need to use sudo for executing this command
    sudo: bool,
}

impl SingleCommand{
    pub fn new(command: String, quiet: bool, sudo: bool) -> Result<Self, String>{

        // Remove useless whitespaces, so we can check if sudo is inside the command
        let command = command.trim().to_string();

        // Check if command string has sudo
        if command.starts_with("sudo"){
            return Err("Given command string has sudo, that has to be specified with a boolean".to_string());
        }

        return Ok(Self{command, quiet, sudo});
    }

    /// Runs the command
    pub fn run(&self) -> Result<(), String>{

        // Get the builder of the command
        let mut builder = self.get_builder_command();

        // Spawn the command and get the handler
        let handler = match builder.spawn(){
            Err(err) => return Err("Command failed to execute".to_string()),
            Ok(child) => child,
        };

        return Ok(());
    }

    /// Creates the `Command` struct, that we can use for spawning, getting the output, ...
    fn get_builder_command(&self) -> Command {

        // Get all the parts of the command
        let mut string_parts: Vec<&str> = self.command.split(" ").collect();

        // Construct the builder `Command`
        let mut builder = Command::new(string_parts[0]);
        for arg in string_parts.iter().skip(0){
            builder.arg(arg);
        }

        return builder;
    }
}


/// Tests associated with the run of a single command
#[cfg(test)]
mod single_command_test{
    use super::SingleCommand;

    #[test]
    pub fn test_failing_command() -> Result<(), String>{
        // Build and run a failing command
        let command = SingleCommand::new(
            "This command does not exist".to_string(), false, false
        )?;
        let result = command.run();

        // Check that the command failed to run
        match result{
            Ok(value) => return Err(format!("Expected error, obtained {:?}", value)),
            Err(_) => return Ok(()),
        }
    }

    #[test]
    pub fn test_basic_command_runs() -> Result<(), String>{
        // Build and run a failing command
        let command = SingleCommand::new(
            "ls -la".to_string(), false, false
        )?;

        let result = command.run();

        // Check that the command didn't have problems running
        match result{
            Ok(_) => return Ok(()),
            Err(err) => return Err(format!("Command failed to run. Error code was: {:?}", err))
        }
    }

    #[test]
    pub fn test_install_unexisting_package_fails() -> Result<(), String>{
        // Build and run a failing command
        let command = SingleCommand::new(
            "pacman -S thispackagedoesnotexist".to_string(), false, true
        )?;

        let result = command.run();

        // Check that the install command failed to execute
        match result{
            Ok(_) => return Err(format!("Installation of thispackagedoesnotexist run succesfully")),
            Err(_) => return Ok(()),
        }
    }
}
