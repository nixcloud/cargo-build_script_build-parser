use std::fs;
use std::path::PathBuf;
use clap::Parser;
use regex::Regex;
use colored::*;

mod tests;

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
    file_path: String,
}

#[derive(clap::Subcommand)]
enum Command {
    #[clap(about = "Parse rustc arguments from build.rs output")]
    RustcArguments,
    #[clap(about = "Parse rustc arguments from build.rs output")]
    RustcPropagatedArguments,
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

fn eprintln_document_with_error(content: String, error_line: usize) {
    for (line_number, line) in content.lines().enumerate() {
        let formatted_line_number = format!("{:3}   ", line_number);
        if line_number == error_line {
            eprintln!("> {} {}", formatted_line_number, line.red());
        } else {
            eprintln!("  {} {}", formatted_line_number, line);
        }
    }
}

fn handle_content(c: Command, content: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut rustc_arguments: Vec<String> = vec![];
    let mut rustc_propagated_arguments: Vec<String> = vec![];
    let mut environment_variables: Vec<String> = vec![];
    for (line_number, line) in content.lines().enumerate() {

        let line = line.trim(); // Remove any trailing newline or whitespace
        let re = Regex::new(r"^cargo:([^=]+)\s*=\s*(.+)$")
            .map_err(|e| format!("Regex error: {}", e))?;
    
        if let Some(caps) = re.captures(line) {
            let command = &caps[1];
            let arg = &caps[2];

            match command {
                // rustc
                "rustc-cfg" => rustc_arguments.push(format!("--cfg '{}'", arg)),
                "rustc-check-cfg" => rustc_arguments.push(format!("--check-cfg '{}'", arg)),
    
                // env
                "rustc-env" => environment_variables.push(format!("{}", arg)),
    
                "warning" => {
                    eprintln!("\x1b[1;33mwarning\x1b[0m: {arg}");
                },
                "error" => {
                    eprintln!("\x1b[1;31error\x1b[0m: {arg}");
                },
    
                // cargo:rustc-link-lib=static=sqlite3
                "rustc-link-lib" => rustc_arguments.push(format!("-l '{}'", arg)),
                // cargo:rustc-link-search=native=/build/tmp.X3Lovygu3U
                "rustc-link-search" => {
                    rustc_propagated_arguments.push(format!("-L '{}'", arg));
                    let re = Regex::new(r"^(.+)\s*=\s*(.+)$")
                        .map_err(|e| format!("Regex error: {}", e))?;
                    if let Some(caps) = re.captures(&arg) {
                        let mode = &caps[1];
                        let _directory = &caps[2];
                        rustc_arguments.push(format!("-L \"{}=$out\"", mode));
                    } else {
                        eprintln_document_with_error(content.clone(), line_number);
                        return Err(format!("Unable to parse rustc-link-search argument at {line_number}: '{line}'").to_string().into())
                    }
                },
                // FIXME the lib_dir is probably implemented wrong here
                // ignored // cargo:lib_dir=/build/tmp.X3Lovygu3U
                "lib_dir" => {}, //rustc_arguments.push(format!("-L $out")),
    
                // ignored // cargo:include=/build/libsqlite3-sys-0.31.0/sqlite3
                "include" => {},
                // ignored // cargo:conf=OPENSSL_NO_SSL3_METHOD
                "conf" => {},
                // ignored // cargo:version_number=30400010
                "version_number" => {},
    
                // ignored
                "rerun-if-changed" => {},
                "rerun-if-env-changed" => {}, 
                "rerun-if-changed-bin" => {},
                "rerun-if-changed-glob" => {},
                "rerun-if-changed-dir" => {},
                "rerun-if-changed-recursive" => {},
                "rerun-if-changed-env" => {},
    
                // fail
                "rustc-flags" |
                "rustc-cdylib-link-arg" |
                "rustc-bin-link-arg" |
                "rustc-link-arg-bin" |
                _ => {
                    eprintln_document_with_error(content.clone(), line_number);
                    return Err(format!("Command: '{command}' on line: '{line_number}' not implemented yet!").into())
                },
            }

        } else {
            eprintln_document_with_error(content.clone(), line_number);
            return Err((format!("Unable to parse the line {line_number}: '{line}'").to_string()).into())
        };
    }

    match c {
        Command::RustcArguments => Ok(format!("{}", rustc_arguments.join(" "))),
        Command::RustcPropagatedArguments => Ok(format!("{}", rustc_propagated_arguments.join(" "))),
        Command::EnvironmentVariables => Ok(format!("{}", environment_variables.join("\n"))),
    }
}
