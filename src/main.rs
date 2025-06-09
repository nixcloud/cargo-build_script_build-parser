use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        eprintln!("Usage: parse-build --tool <tool> --command <command> <file_path>");
        println!("{}", args.len());
        std::process::exit(1);
    }

    let tool = &args[2];
    let command = &args[4];
    let file_path = PathBuf::from(&args[5]);

    if tool != "cargo" {
        eprintln!("Only cargo tool is supported.");
        std::process::exit(1);
    }

    let content = fs::read_to_string(&file_path).expect("Could not read file");

    let output = parse_command_output(command, &content);

    println!("{}", output.trim_end());
}

fn parse_command_output(command: &str, content: &str) -> String {
    let mut output = String::new();

    match command {
        "rustc-cfg" => {
            for line in content.lines() {
                if let Some(value) = line.strip_prefix("cargo:rustc-cfg=") {
                    output.push_str(&format!("--cfg='{}' ", value));
                }
            }
        }
        "rustc-check-cfg" => {
            for line in content.lines() {
                if let Some(value) = line.strip_prefix("cargo:rustc-check-cfg=") {
                    output.push_str(&format!("--check-cfg='{}' ", value));
                }
            }
        }
        _ => {
            eprintln!("Unsupported command: {}", command);
            std::process::exit(1);
        }
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
        let output = parse_command_output("rustc-cfg", &content);
        assert!(output.contains("--cfg='freebsd11'"));
        assert!(output.contains("--cfg='libc_const_extern_fn'"));
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
        let output = parse_command_output("rustc-check-cfg", &content);
        assert!(output.contains("--check-cfg='cfg(espidf_time32)'"));
        assert!(output.contains("--check-cfg='cfg(libc_ctest)'"));
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

        let cfg_output = parse_command_output("rustc-cfg", &content);
        assert_eq!(
            cfg_output.trim(),
            "--cfg='freebsd11' --cfg='libc_const_extern_fn'"
        );

        let check_cfg_output = parse_command_output("rustc-check-cfg", &content);
        assert_eq!(
            check_cfg_output.trim(),
            "--check-cfg='cfg(libc_ctest)' --check-cfg='cfg(target_arch,values(\"mips64r6\"))'"
        );
    }
}
