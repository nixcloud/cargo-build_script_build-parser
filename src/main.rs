use std::env;
use std::fs;
use std::path::PathBuf;

mod tests;

enum Command {
    RustcCfg,
    RustcCheckCfg,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        eprintln!(
            "Usage: parse-build --tool <tool> --command <command> <file_path>\n\
                   Supported commands: {}",
            Command::variants()
        );
        std::process::exit(1);
    }

    let tool = &args[2];
    let command_str = &args[4];
    let file_path = PathBuf::from(&args[5]);

    if tool != "cargo" {
        eprintln!("Only cargo tool is supported.");
        std::process::exit(1);
    }

    match Command::parse(command_str) {
        Ok(cmd) => {
            let content = fs::read_to_string(&file_path).expect("Could not read file");
            let output = parse_command_output(cmd, &content);
            println!("{}", output.trim_end());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

impl Command {
    fn variants() -> String {
        format!("cargo-rustc-cfg | cargo-rustc-check-cfg")
    }

    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "rustc-cfg" => Ok(Command::RustcCfg),
            "rustc-check-cfg" => Ok(Command::RustcCheckCfg),
            _ => Err(format!(
                "Unsupported command. Supported commands: {}",
                Command::variants()
            )),
        }
    }
}

fn parse_command_output(cmd: Command, content: &str) -> String {
    let mut output = String::new();

    match cmd {
        Command::RustcCfg => {
            for line in content.lines() {
                if let Some(value) = line.strip_prefix("cargo:rustc-cfg=") {
                    output.push_str(&format!("--cfg='{}' ", value));
                }
            }
        }
        Command::RustcCheckCfg => {
            for line in content.lines() {
                if let Some(value) = line.strip_prefix("cargo:rustc-check-cfg=") {
                    output.push_str(&format!("--check-cfg='{}' ", value));
                }
            }
        }
    }

    output
}
