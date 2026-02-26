use std::env;
use std::process::{Command, Stdio};

fn cwd() -> String {
    env::current_dir()
        .map(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "/".to_string())
        })
        .unwrap_or_else(|_| "/".to_string())
}

/// Plain-text prompt — passed to readline() for correct width calculation.
pub fn build() -> String {
    let cwd = cwd();
    match git_branch() {
        Some(branch) => format!("~ {} git:({}) $ ", cwd, branch),
        None => format!("~ {} $ ", cwd),
    }
}

/// Colored prompt — returned from highlight_prompt() for display only.
#[cfg(not(windows))]
pub fn build_colored() -> String {
    let cwd = cwd();
    match git_branch() {
        Some(branch) => format!(
            "~ \x1b[34m{}\x1b[0m git:(\x1b[31m{}\x1b[0m) $ ",
            cwd, branch
        ),
        None => format!("~ \x1b[34m{}\x1b[0m $ ", cwd),
    }
}

#[cfg(windows)]
pub fn build_colored() -> String {
    build()
}

fn git_branch() -> Option<String> {
    let out = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if out.status.success() {
        let branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if branch.is_empty() { None } else { Some(branch) }
    } else {
        None
    }
}
