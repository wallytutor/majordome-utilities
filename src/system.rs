pub fn run_command(
    log: &std::fs::File,
    cmd: &str,
    args: &[&str],
) -> Result<std::process::ExitStatus, std::io::Error> {
    std::process::Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::from(log.try_clone()?))
        .stderr(std::process::Stdio::from(log.try_clone()?))
        .status()
}

/// Resolve entrypoint arguments from Python wrappers or process CLI.
///
/// Contract: returned vector contains CLI arguments only (no program name).
pub fn resolve_args(args: Option<Vec<String>>) -> Vec<String> {
    args.unwrap_or_else(|| std::env::args().skip(1).collect())
}

pub fn remove_file_if_exists(file_path: &std::path::Path) {
    if file_path.exists() {
        match std::fs::remove_file(file_path) {
            Ok(_) => print_success!("removed existing {}", file_path.display()),
            Err(e) => print_error!("failed to remove existing {}: {}", file_path.display(), e),
        }
    } else {
        print_warning!("no existing {} file found", file_path.display());
    }
}
