use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::shell::Shell;

pub const BUILTINS: &[&str] = &[
    "cd", "clear", "echo", "env", "exit", "export", "pwd", "type", "unset", "which",
];

pub fn is_builtin(name: &str) -> bool {
    BUILTINS.contains(&name)
}

/// Returns the exit code. A value of -1 signals the shell to exit.
pub fn run(argv: &[String], shell: &mut Shell) -> i32 {
    match argv.first().map(String::as_str) {
        Some("exit") => {
            let code = argv.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            // Save history is handled in main before we exit, so just signal exit.
            // We use a sentinel that the caller understands.
            shell.last_exit_code = code;
            -1 // sentinel: exit shell
        }
        Some("echo") => {
            println!("{}", argv[1..].join(" "));
            0
        }
        Some("pwd") => match env::current_dir() {
            Ok(p) => { println!("{}", p.display()); 0 }
            Err(e) => { eprintln!("pwd: {}", e); 1 }
        },
        Some("cd") => {
            let path = match argv.get(1) {
                Some(p) => p.clone(),
                None => env::var("HOME").unwrap_or_else(|_| "/".to_string()),
            };
            let target = resolve_path(&path);
            match env::set_current_dir(&target) {
                Ok(_) => 0,
                Err(e) => { eprintln!("cd: {}: {}", target.display(), e); 1 }
            }
        }
        Some("clear") => {
            print!("\x1b[2J\x1b[H");
            io::stdout().flush().ok();
            0
        }
        Some("export") => {
            for arg in &argv[1..] {
                if let Some((k, v)) = arg.split_once('=') {
                    unsafe { env::set_var(k, v) };
                }
            }
            0
        }
        Some("unset") => {
            for arg in &argv[1..] {
                unsafe { env::remove_var(arg) };
            }
            0
        }
        Some("env") => {
            for (k, v) in env::vars() {
                println!("{}={}", k, v);
            }
            0
        }
        Some("which") => {
            let mut code = 0;
            for arg in &argv[1..] {
                match find_in_path(arg) {
                    Some(p) => println!("{}", p.display()),
                    None => { eprintln!("{}: not found", arg); code = 1; }
                }
            }
            code
        }
        Some("type") => {
            let mut code = 0;
            for arg in &argv[1..] {
                if is_builtin(arg) {
                    println!("{} is a shell builtin", arg);
                } else if let Some(p) = find_in_path(arg) {
                    println!("{} is {}", arg, p.display());
                } else {
                    eprintln!("{}: not found", arg);
                    code = 1;
                }
            }
            code
        }
        _ => 127,
    }
}

pub fn find_in_path(name: &str) -> Option<PathBuf> {
    if name.contains('/') {
        let p = PathBuf::from(name);
        return if p.is_file() { Some(p) } else { None };
    }
    let path_var = env::var("PATH").unwrap_or_default();
    for dir in path_var.split(':') {
        let candidate = Path::new(dir).join(name);
        if candidate.is_file() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(meta) = candidate.metadata() {
                    if meta.permissions().mode() & 0o111 != 0 {
                        return Some(candidate);
                    }
                }
                continue;
            }
            #[cfg(not(unix))]
            return Some(candidate);
        }
    }
    None
}

fn resolve_path(path: &str) -> PathBuf {
    if path == "~" || path.starts_with("~/") {
        let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        PathBuf::from(format!("{}{}", home, &path[1..]))
    } else {
        PathBuf::from(path)
    }
}
