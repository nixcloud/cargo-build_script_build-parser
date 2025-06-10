use std::env;
use std::fs;
use std::path::PathBuf;

enum Command {
    RustcCfg,
    RustcCheckCfg,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        eprintln!("Usage: parse-build --tool <tool> --command <command> <file_path>\n\
                   Supported commands: {}",
                 Command::variants());
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
        },
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
        },
        Command::RustcCheckCfg => {
            for line in content.lines() {
                if let Some(value) = line.strip_prefix("cargo:rustc-check-cfg=") {
                    output.push_str(&format!("--check-cfg='{}' ", value));
                }
            }
        },
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_rustc_cfg_output() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(
            temp,
            "cargo:rustc-cfg=freebsd11\ncargo:rustc-cfg=libc_const_extern_fn"
        )
        .unwrap();

        let content = fs::read_to_string(temp.path()).unwrap();
        let output = parse_command_output(Command::parse("rustc-cfg").unwrap(), &content);
        assert_eq!(
            output.trim(),
            "--cfg='freebsd11' --cfg='libc_const_extern_fn'"
        );
    }
    #[test]
    fn test_rustc_check_cfg_output() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(
            temp,
            "cargo:rustc-check-cfg=cfg(espidf_time32)\ncargo:rustc-check-cfg=cfg(libc_ctest)"
        )
        .unwrap();

        let content = fs::read_to_string(temp.path()).unwrap();
        let output = parse_command_output(Command::parse("rustc-check-cfg").unwrap(), &content);
        assert_eq!(
            output.trim(),
            "--check-cfg='cfg(espidf_time32)' --check-cfg='cfg(libc_ctest)'"
        );
    }

    #[test]
    fn test_mixed_content() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(
            temp,
            "cargo:rustc-cfg=freebsd11\n\
             cargo:rustc-check-cfg=cfg(libc_ctest)\n\
             cargo:rustc-cfg=libc_const_extern_fn\n\
             cargo:rustc-check-cfg=cfg(target_arch,values(\"mips64r6\"))"
        )
        .unwrap();

        let content = fs::read_to_string(temp.path()).unwrap();

        let cfg_output = parse_command_output(Command::parse("rustc-cfg").unwrap(), &content);
        assert_eq!(
            cfg_output.trim(),
            "--cfg='freebsd11' --cfg='libc_const_extern_fn'"
        );

        let check_cfg_output = parse_command_output(Command::parse("rustc-check-cfg").unwrap(), &content);
        assert_eq!(
            check_cfg_output.trim(),
            "--check-cfg='cfg(libc_ctest)' --check-cfg='cfg(target_arch,values(\"mips64r6\"))'"
        );
}
}
