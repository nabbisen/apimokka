use std::process::Command;

/// run command-line in shell to wait for finished and get the output
pub fn command_output(command: &str) -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            // utf-8 specified
            .args(&["/C", "chcp 65001"])
            .output()
            .expect("Failed to run chcp command");
        Command::new("cmd").args(&["/C", command]).output()
    } else {
        Command::new("sh").args(&["-c", command]).output()
    };
    match output {
        Ok(output) => {
            if output.status.success() {
                let s = String::from_utf8(output.stdout);
                match s {
                    Ok(s) => Ok(s),
                    Err(err) => Err(err.to_string()),
                }
            } else {
                let s = String::from_utf8(output.stderr);
                match s {
                    Ok(s) => Err(s),
                    Err(err) => Err(err.to_string()),
                }
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

/// open file with the default (associated-with) app
pub fn open_with_default_app(filepath: &str) -> Result<String, String> {
    let command = if cfg!(target_os = "windows") {
        format!("start \"\" \"{}\"", filepath)
    } else if cfg!(target_os = "macos") {
        format!("open {}", filepath)
    } else if cfg!(target_os = "linux") {
        format!("xdg-open {}", filepath)
    } else {
        return Err("Unsupported OS".to_owned());
    };
    command_output(command.as_str())
}
