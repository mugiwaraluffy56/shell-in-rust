#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env};
// use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
use std::process::Command;

enum Commands {
    Echo(Vec<String>),
    Type(String),
    Exit,
    Unknown(String),
    Clear,
    Cd(String),
    Pwd,
}

fn main() {

    loop {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .expect("Failed to execute git command");
        let branch = String::from_utf8_lossy(&output.stdout);

        match env::current_dir() {
            Ok(value) => print!("~ \x1b[34m{}\x1b[0m", value.file_name().unwrap().to_string_lossy()),
            Err(_) => print!("/"),
        }
        print!(" git:(\x1b[31m{}\x1b[0m) $ ", branch.trim());
        io::stdout().flush().unwrap();
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let args = input.trim().split_whitespace().collect::<Vec<&str>>();

        // let mut command = String::new();
        // io::stdin().read_line(&mut command).unwrap();
        // println!("{}: command not found", command.trim());

        let command = match args.as_slice() {
            ["exit"] => Commands::Exit,
            ["echo", msg @ ..] => {
                Commands::Echo(msg.iter().map(|s| s.to_string()).collect())

            }
            ["type", cmd] => Commands::Type(cmd.to_string()),
            ["clear"] => Commands::Clear,
            ["pwd"] => Commands::Pwd,
            ["cd"] => {
                let home = env::home_dir().unwrap_or_else(|| "/".into());
                Commands::Cd(home.to_string_lossy().to_string())
            },
            ["cd", path] => Commands::Cd(path.to_string()),
            [cmd, ..] => Commands::Unknown(cmd.to_string()),
            [] => continue,
        };

        match command {
            Commands::Exit => {
                println!("Session exited successfully!");
                break;
            },

            Commands::Echo(msg) => {
                println!("{}", msg.join(" "))
            }

            Commands::Type(cmd) => {
                if ["exit", "echo", "type"].contains(&cmd.as_str()) {
                    println!("{} is a shell builtin", cmd);
                    continue;
                }
            }
            Commands::Unknown(cmd) => {
                println!("{}: command not found", cmd);
            }
            
            Commands::Clear => {
                Command::new("clear")
                    .status()
                    .unwrap();
            }

            Commands::Cd(path) => {
                let target_dir = if path.starts_with("~") {
                    let home = env::home_dir().unwrap_or_else(|| "/".into());
                    home.join(&path[1..])
                } else {
                    path.into()
                };

                if let Err(e) = env::set_current_dir(&target_dir) {
                    eprintln!("cd: {}: {}", target_dir.display(), e);
                }
            }

            Commands::Pwd => {
                match env::current_dir() {
                    Ok(value) => println!("{} ", value.display()),
                    Err(_) => print!("/"),
                }
            }
        }
        
    }
}