use std::process::{Command, Stdio};
use std::io::Read;

pub fn execute_command(command: &str) -> Result<(), std::io::Error> {
    let mut child = Command::new("sh") // Use a shell explicitly
        .arg("-c") // Specify that you want to execute a command
        .arg(command) // The actual command to execute
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdout = String::new();
    let mut stderr = String::new();

    child.stdout.take().unwrap().read_to_string(&mut stdout)?;
    child.stderr.take().unwrap().read_to_string(&mut stderr)?;

    let status = child.wait()?;

    if !status.success() {
        // Command execution failed, return an error with stderr information
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command failed with exit code: {}\n{}", status, stderr),
        ));
    }

    Ok(())
}
