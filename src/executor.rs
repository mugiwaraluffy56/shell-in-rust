use std::fs::{File, OpenOptions};
use std::process::{Command, Stdio};

use crate::builtins;
use crate::parser::{CommandList, Pipeline, SimpleCmd};
use crate::shell::Shell;

/// Run all pipelines in the list sequentially.
/// Returns false if the shell should exit.
pub fn run_list(list: CommandList, shell: &mut Shell) -> bool {
    for pipeline in list {
        let code = run_pipeline(pipeline, shell);
        if code == -1 {
            return false; // exit sentinel
        }
        shell.last_exit_code = code;
    }
    true
}

fn run_pipeline(pipeline: Pipeline, shell: &mut Shell) -> i32 {
    if pipeline.is_empty() {
        return 0;
    }

    // Single command: run builtins in-process
    if pipeline.len() == 1 {
        let cmd = &pipeline[0];
        if !cmd.argv.is_empty() && builtins::is_builtin(&cmd.argv[0]) {
            return builtins::run(&cmd.argv, shell);
        }
        return spawn_single(cmd);
    }

    // Multi-command pipeline
    run_pipe_chain(pipeline)
}

fn spawn_single(cmd: &SimpleCmd) -> i32 {
    if cmd.argv.is_empty() {
        return 0;
    }

    if builtins::find_in_path(&cmd.argv[0]).is_none() {
        eprintln!("{}: command not found", cmd.argv[0]);
        return 127;
    }

    let stdin = make_stdin(&cmd.stdin_file);
    let stdout = make_stdout(&cmd.stdout_file, cmd.append);

    match Command::new(&cmd.argv[0])
        .args(&cmd.argv[1..])
        .stdin(stdin)
        .stdout(stdout)
        .spawn()
    {
        Ok(mut child) => child.wait().map(|s| s.code().unwrap_or(0)).unwrap_or(1),
        Err(e) => { eprintln!("{}: {}", cmd.argv[0], e); 1 }
    }
}

fn run_pipe_chain(pipeline: Pipeline) -> i32 {
    let last = pipeline.len() - 1;
    let mut prev_stdout: Option<std::process::ChildStdout> = None;
    let mut children: Vec<std::process::Child> = Vec::new();

    for (i, cmd) in pipeline.iter().enumerate() {
        if cmd.argv.is_empty() {
            continue;
        }

        let is_last = i == last;

        let stdin: Stdio = if let Some(out) = prev_stdout.take() {
            Stdio::from(out)
        } else if let Some(ref f) = cmd.stdin_file {
            match File::open(f) {
                Ok(file) => Stdio::from(file),
                Err(e) => { eprintln!("{}: {}", f, e); return 1; }
            }
        } else {
            Stdio::inherit()
        };

        let stdout: Stdio = if is_last {
            make_stdout(&cmd.stdout_file, cmd.append)
        } else {
            Stdio::piped()
        };

        match Command::new(&cmd.argv[0])
            .args(&cmd.argv[1..])
            .stdin(stdin)
            .stdout(stdout)
            .spawn()
        {
            Ok(mut child) => {
                if !is_last {
                    prev_stdout = child.stdout.take();
                }
                children.push(child);
            }
            Err(e) => {
                eprintln!("{}: {}", cmd.argv[0], e);
                // Wait for already-spawned children before returning
                for c in &mut children { c.wait().ok(); }
                return 1;
            }
        }
    }

    let last_idx = children.len().saturating_sub(1);
    let mut last_code = 0;
    for (i, child) in children.iter_mut().enumerate() {
        if let Ok(status) = child.wait() {
            if i == last_idx {
                last_code = status.code().unwrap_or(0);
            }
        }
    }
    last_code
}

fn make_stdin(file: &Option<String>) -> Stdio {
    match file {
        None => Stdio::inherit(),
        Some(f) => match File::open(f) {
            Ok(file) => Stdio::from(file),
            Err(e) => { eprintln!("{}: {}", f, e); Stdio::inherit() }
        }
    }
}

fn make_stdout(file: &Option<String>, append: bool) -> Stdio {
    match file {
        None => Stdio::inherit(),
        Some(f) => {
            let result = if append {
                OpenOptions::new().create(true).append(true).open(f)
            } else {
                OpenOptions::new().create(true).write(true).truncate(true).open(f)
            };
            match result {
                Ok(file) => Stdio::from(file),
                Err(e) => { eprintln!("{}: {}", f, e); Stdio::inherit() }
            }
        }
    }
}
