use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::env;

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

/// Enum to represent all kind of errors that could happen
#[derive(Debug)]
pub enum SingleCommandError{

    /// Error that is returned when the command string has sudo
    /// If sudo is wanted to be used, it has to be set in the sudo bool field
    SudoAtTheStart,

    /// Error that is returned when the program does not exist. Thus, this error
    /// raises when we try to create a command with this unexisting program
    ProgramDoesNotExist(String),

    /// Error that is returned when the program fails in runtime
    /// That is to say, when the program runs but in the middle of execution fails
    RuntimeFailure(String),
}

impl SingleCommand{
    pub fn new(command: String, quiet: bool, sudo: bool) -> Result<Self, SingleCommandError>{

        // Remove useless whitespaces, so we can check if sudo is inside the command
        let command = command.trim().to_string();

        // Check if command string has sudo
        if command.starts_with("sudo"){
            return Err(SingleCommandError::SudoAtTheStart);
        }

        return Ok(Self{command, quiet, sudo});
    }

    /// Runs the command
    pub fn run(&self) -> Result<(), SingleCommandError>{

        // Get the builder of the command
        let mut builder = self.get_builder_command();

        // Spawn the command and get the handler
        let mut handler = match builder.spawn(){

            // If this fails when creating the handle, it's because the program does not exist
            Err(err) => return Err(SingleCommandError::ProgramDoesNotExist(format!("{:?}", err))),

            Ok(child) => child,
        };

        // Wait the command to run and capture the exit code
        let exit_code = match handler.wait(){
            Err(err) => return Err(SingleCommandError::RuntimeFailure(format!("{:?}", err))),
            Ok(exit_code) => exit_code,
        };

        // Check that exit code
        let code = match exit_code.code(){
            None => return Err(SingleCommandError::RuntimeFailure("Process terminated by signal".to_string())),
            Some(code) => code,
        };

        if code != 0{
            return Err(SingleCommandError::RuntimeFailure(format!("Exit code was not zero, was {}", code)));
        }

        // Everything went ok
        return Ok(());
    }

    /// Creates the `Command` struct, that we can use for spawning, getting the output, ...
    fn get_builder_command(&self) -> Command {

        // Get all the parts of the command
        let mut string_parts: Vec<&str> = self.command.split(" ").collect();

        // Use sudo if specified
        if self.sudo == true{
            string_parts.insert(0, "sudo");
        }

        // Construct the builder `Command`
        let mut builder = Command::new(string_parts[0]);
        for (index, arg) in string_parts.iter().enumerate(){

            // We have already used first arg
            if index == 0{
                continue;
            }

            builder.arg(arg);
        }

        // Inherit the stdio so we can get user input
        builder.stdin(Stdio::inherit());

        // Do not show the output if specified as such
        if self.quiet == true{
            builder.stdout(Stdio::null());
        }

        return builder;
    }
}


/// Tests associated with the run of a single command
#[cfg(test)]
mod single_command_test{
    use super::{SingleCommand, SingleCommandError};

    #[test]
    pub fn test_failing_command_with_unexisting_program() -> Result<(), String>{

        // Build and run a failing command
        let command = SingleCommand::new(
            "This command does not exist".to_string(), false, false
        ).expect("This command doesn't have sudo at the start");
        let result = command.run();

        // Check that the command failed to run because the program does not exist
        match result {
            Ok(value) => return Err(format!("Expected error, obtained {:?}", value)),
            Err(SingleCommandError::SudoAtTheStart) => return Err("This command should faild because the program doesn't exist, not because it has sudo at the start".to_string()),
            Err(SingleCommandError::RuntimeFailure(_)) => return Err("This command should faild because the program doesn't exist, not because it has some runtime failure".to_string()),
            Err(SingleCommandError::ProgramDoesNotExist(_)) => return Ok(()),
        }
    }

    #[test]
    pub fn test_basic_command_runs() -> Result<(), String>{
        // Build and run a failing command
        let command = SingleCommand::new(
            "ls -la".to_string(), false, false
        ).expect("This command doesn't have sudo at the start");

        let result = command.run();

        // Check that the command didn't have problems running
        match result{
            Ok(_) => return Ok(()),
            Err(err) => return Err(format!("Command failed to run. Error code was: {:?}", err))
        }
    }

    #[test]
    pub fn test_failing_command_in_runtime() -> Result<(), String>{
        // Build and run a failing command
        let command = SingleCommand::new(
            "ls -lah /dev/thisdirdoesnotexist".to_string(), false, true
        ).expect("This command doesn't have sudo at the start");

        let result = command.run();

        // Check that the install command failed to execute
        // And that the failure is because something with the execution failed
        match result{
            Ok(_) => return Err(format!("Installation of thispackagedoesnotexist run succesfully")),
            Err(SingleCommandError::SudoAtTheStart) => return Err("This command should fail in runtime, not because it has sudo".to_string()),
            Err(SingleCommandError::ProgramDoesNotExist(_)) => return Err("This command should fail in runtime, not because the program does not exist".to_string()),
            Err(SingleCommandError::RuntimeFailure(_)) => return Ok(()),
        }
    }

    #[test]
    pub fn test_sudo_in_command_fails() -> Result<(), String>{
        // Build and run a failing command
        let command_result = SingleCommand::new(
            "sudo ls -lah /dev/thisdirdoesnotexist".to_string(), false, false
        );

        match command_result{
            Ok(_) => return Err("Command has sudo in the string command and thus should not build".to_string()),
            Err(_) => return Ok(()),
        }
    }

    #[test]
    pub fn test_sudo_command_can_run() -> Result<(), SingleCommandError>{
        // Run a command with sudo
        let command = SingleCommand::new(
            "ls /etc/".to_string(), false, true
        )?;
        command.run()?;

        // Everything went ok
        return Ok(());

    }
}
