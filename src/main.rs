use std::fs;
use std::path::PathBuf;
use clap::Parser;
use regex::Regex;

mod tests;

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
    file_path: String,
}

#[derive(clap::Subcommand)]
enum Command {
    #[clap(about = "Parse rustc arguments from cargo output")]
    RustcArguments,
    #[clap(about = "Parse environment variables from cargo output")]
    EnvironmentVariables,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let content = fs::read_to_string(PathBuf::from(&args.file_path)).expect("Could not read file");
    match handle_content(args.command, content) {
        Ok(out) => println!("{out}"),
        Err(e) => return Err(e)
    }
    Ok(())
}

fn handle_content(c: Command, content: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut rustc_arguments: Vec<String> = vec![];
    let mut environment_variables: Vec<String> = vec![];
    for (line_number, line) in content.lines().enumerate() {
        let (command, arg) = parse(line_number, line)?;
        match command.as_str() {
            // rustc
            "rustc-cfg" => rustc_arguments.push(format!("--cfg={}", arg)),
            "rustc-check-cfg" => rustc_arguments.push(format!("--check-cfg={}", arg)),

            // env
            "rustc-env" => environment_variables.push(format!("{}", arg)),

            // warning
            "warning" => {
                eprintln!("WARNING: {arg}");
            },

            // ignored
            "rerun-if-changed" => {},
            "rerun-if-env-changed" => {}, 
            "rerun-if-changed-bin" => {},
            "rerun-if-changed-glob" => {},
            "rerun-if-changed-dir" => {},
            "rerun-if-changed-recursive" => {},
            "rerun-if-changed-env" => {},

            // fail
            "rustc-link-lib" |
            "rustc-link-search" |
            "rustc-flags" |
            "rustc-cdylib-link-arg" |
            "rustc-bin-link-arg" |
            "rustc-link-arg-bin" => {
                return Err(format!("Command: '{command}' on line: '{line_number}' not implemented yet!").into())
            },

            _ => {
                return Err(format!("Unexpected command: '{command}' on line: '{line_number}'").into())
            },
        }
    }

    match c {
        Command::RustcArguments => Ok(format!("{}", rustc_arguments.join(" "))),
        Command::EnvironmentVariables => Ok(format!("{}", environment_variables.join("\n"))),
    }
}

fn parse(line_number: usize, line: &str) -> Result<(String,String), String> {
    let line = line.trim(); // Remove any trailing newline or whitespace
    let re = Regex::new(r"^cargo:([^=]+)\s*=\s*(.+)$")
        .map_err(|e| format!("Regex error: {}", e))?;

    if let Some(caps) = re.captures(line) {
        let command = &caps[1];
        let arg = &caps[2];
        Ok((command.to_string(), arg.to_string()))
    } else {
        Err(format!("Unable to parse the line {line_number}: '{line}'").to_string())
    }
}
