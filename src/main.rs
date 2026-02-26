
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env};
use std::fs::metadata;
// use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

enum Commands {
    Echo(Vec<String>),
    Type(String),
    Exit,
    Unknown(String),
}

fn main() {

    loop {
        match env::current_dir() {
            Ok(value) => print!("{}", value.file_name().unwrap().to_string_lossy()),
            Err(_) => print!("/"),
        }
        print!(" $ ");
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

            // Commands::Type(cmd) => match cmd.as_str() {
                // "exit" | "echo" | "type" => {
                    // println!("{} is a shell builtin", cmd);
                // }
                // _ => println!("{}: not found", cmd),
            // },

            Commands::Type(cmd) => {
                if ["exit", "echo", "type"].contains(&cmd.as_str()) {
                    println!("{} is a shell builtin", cmd);
                    continue;
                }

                if let Ok(path_var) = env::var("PATH") {
                    let paths = env::split_paths(&path_var);

                    for dir in paths {
                        let full_path = dir.join(&cmd);

                        if let Ok(metadata) = metadata(&full_path) {

                            if metadata.is_file() {
                                #[cfg(unix)]
                                {
                                    if metadata.permissions().mode() & 0o111 != 0 {
                                        println!("{} is {}", cmd, full_path.display());
                                    break;
                                    }         
                                }
                            }
                        }
                    }
                }
            }
            Commands::Unknown(cmd) => {
                println!("{}: command not found", cmd);
            }
        }
    }
}