use crate::lexer::Token;

/// One command with optional redirections.
#[derive(Debug, Clone, Default)]
pub struct SimpleCmd {
    pub argv: Vec<String>,
    pub stdin_file: Option<String>,
    pub stdout_file: Option<String>,
    pub append: bool,
}

/// A sequence of commands connected by pipes.
pub type Pipeline = Vec<SimpleCmd>;

/// A sequence of pipelines separated by semicolons.
pub type CommandList = Vec<Pipeline>;

pub fn parse(tokens: Vec<Token>) -> CommandList {
    let mut list: CommandList = Vec::new();
    let mut pipeline: Pipeline = Vec::new();
    let mut cmd = SimpleCmd::default();

    for token in tokens {
        match token {
            Token::Word(w) => cmd.argv.push(w),
            Token::Pipe => {
                if !cmd.argv.is_empty() || cmd.stdin_file.is_some() {
                    pipeline.push(cmd);
                    cmd = SimpleCmd::default();
                }
            }
            Token::Semicolon => {
                if !cmd.argv.is_empty() || cmd.stdin_file.is_some() {
                    pipeline.push(cmd);
                    cmd = SimpleCmd::default();
                }
                if !pipeline.is_empty() {
                    list.push(pipeline);
                    pipeline = Vec::new();
                }
            }
            Token::RedirectOut(f) => {
                cmd.stdout_file = Some(f);
                cmd.append = false;
            }
            Token::RedirectAppend(f) => {
                cmd.stdout_file = Some(f);
                cmd.append = true;
            }
            Token::RedirectIn(f) => {
                cmd.stdin_file = Some(f);
            }
        }
    }

    if !cmd.argv.is_empty() || cmd.stdin_file.is_some() {
        pipeline.push(cmd);
    }
    if !pipeline.is_empty() {
        list.push(pipeline);
    }
    list
}
