mod builtins;
mod completer;
mod executor;
mod lexer;
mod parser;
mod prompt;
mod shell;

use std::env;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::{CompletionType, Config, Editor};

use completer::{collect_path_commands, ShellHelper};
use shell::Shell;

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

fn history_path() -> PathBuf {
    home_dir()
        .unwrap_or_else(env::temp_dir)
        .join(".shell_history")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).map(String::as_str) == Some("--version") {
        println!("rsh v0.1.1");
        return;
    }

    let mut shell = Shell::new();

    let path_commands = collect_path_commands();
    let helper = ShellHelper::new(path_commands);

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();

    let mut rl: Editor<ShellHelper, DefaultHistory> =
        Editor::with_config(config).expect("failed to init editor");
    rl.set_helper(Some(helper));

    let hist = history_path();
    let _ = rl.load_history(&hist);

    loop {
        let prompt = prompt::build();

        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                rl.add_history_entry(trimmed).ok();

                let tokens = lexer::tokenize(trimmed, shell.last_exit_code);
                let list = parser::parse(tokens);

                if !executor::run_list(list, &mut shell) {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(e) => {
                eprintln!("error: {:?}", e);
                break;
            }
        }
    }

    rl.save_history(&hist).ok();
}
