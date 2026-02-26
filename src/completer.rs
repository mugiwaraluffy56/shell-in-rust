use std::borrow::Cow;
use std::fs;

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

use crate::builtins::BUILTINS;

pub struct ShellHelper {
    pub path_commands: Vec<String>,
    pub colored_prompt: String,
    file_completer: FilenameCompleter,
}

impl ShellHelper {
    pub fn new(path_commands: Vec<String>) -> Self {
        Self { path_commands, colored_prompt: String::new(), file_completer: FilenameCompleter::new() }
    }
}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        // Find the start of the word under the cursor
        let word_start = line[..pos]
            .rfind(|c: char| c.is_whitespace() || c == '|' || c == ';')
            .map(|i| i + 1)
            .unwrap_or(0);
        let word = &line[word_start..pos];

        // Determine if word is in command position
        let before = line[..word_start].trim();
        let is_cmd_pos = before.is_empty()
            || before.ends_with('|')
            || before.ends_with(';');

        if is_cmd_pos && !word.contains('/') && !word.contains('\\') {
            let word_lower = word.to_lowercase();
            let matches: Vec<Pair> = BUILTINS
                .iter()
                .map(|s| s.to_string())
                .chain(self.path_commands.iter().cloned())
                .filter(|cmd| cmd.to_lowercase().starts_with(&word_lower))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: format!("{} ", cmd),
                })
                .collect();

            if !matches.is_empty() {
                return Ok((word_start, matches));
            }
        }

        // Fall back to filename completion
        self.file_completer.complete(line, pos, ctx)
    }
}

impl Helper for ShellHelper {}

impl Highlighter for ShellHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        _prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        Cow::Owned(self.colored_prompt.clone())
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Borrowed(hint)
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        false
    }
}

impl Hinter for ShellHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Validator for ShellHelper {}

/// Scan PATH and return sorted, deduplicated executable names.
pub fn collect_path_commands() -> Vec<String> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    let mut cmds: Vec<String> = Vec::new();

    #[cfg(windows)]
    const PATH_SEP: char = ';';
    #[cfg(not(windows))]
    const PATH_SEP: char = ':';

    for dir in path_var.split(PATH_SEP) {
        let Ok(entries) = fs::read_dir(dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let Ok(meta) = entry.metadata() else { continue };
                if meta.permissions().mode() & 0o111 == 0 {
                    continue;
                }
            }
            #[cfg(windows)]
            {
                let ext = path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                if !["exe", "bat", "cmd", "com"].contains(&ext.as_str()) {
                    continue;
                }
            }
            if let Some(name) = path.file_name() {
                cmds.push(name.to_string_lossy().into_owned());
            }
        }
    }

    cmds.sort_unstable();
    cmds.dedup();
    cmds
}
