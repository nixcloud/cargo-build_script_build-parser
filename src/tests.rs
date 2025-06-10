#[cfg(test)]
mod tests {
    use super::*;
    use crate::Command;
    use crate::parse_command_output;
    use std::env;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
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

        let check_cfg_output =
            parse_command_output(Command::parse("rustc-check-cfg").unwrap(), &content);
        assert_eq!(
            check_cfg_output.trim(),
            "--check-cfg='cfg(libc_ctest)' --check-cfg='cfg(target_arch,values(\"mips64r6\"))'"
        );
    }
}
