
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env};

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
            Commands::Exit => break,

            Commands::Echo(msg) => {
                println!("{}", msg.join(" "))
            }

            Commands::Type(cmd) => match cmd.as_str() {
                "exit" | "echo" | "type" => {
                    println!("{} is a shell builtin", cmd);
                }
                _ => println!("{}: not found", cmd),
            },

            Commands::Unknown(cmd) => {
                println!("{}: command not found", cmd);
            }
        }
    }
}