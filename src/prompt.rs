use std::env;
use std::process::{Command, Stdio};

pub fn build() -> String {
    let cwd = env::current_dir()
        .map(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "/".to_string())
        })
        .unwrap_or_else(|_| "/".to_string());

    #[cfg(windows)]
    return match git_branch() {
        Some(branch) => format!("~ {} git:({}) $ ", cwd, branch),
        None => format!("~ {} $ ", cwd),
    };

    #[cfg(not(windows))]
    match git_branch() {
        Some(branch) => format!(
            "~ \x01\x1b[34m\x02{}\x01\x1b[0m\x02 git:(\x01\x1b[31m\x02{}\x01\x1b[0m\x02) $ ",
            cwd, branch
        ),
        None => format!("~ \x01\x1b[34m\x02{}\x01\x1b[0m\x02 $ ", cwd),
    }
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
