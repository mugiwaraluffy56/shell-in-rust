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

/// How a pipeline is conditionally connected to the previous one.
#[derive(Debug, Clone)]
pub enum RunIf {
    Always,    // first, or after ;
    OnSuccess, // after &&
    OnFailure, // after ||
}

/// A pipeline with its run condition.
pub type CommandList = Vec<(Pipeline, RunIf)>;

pub fn parse(tokens: Vec<Token>) -> CommandList {
    let mut list: CommandList = Vec::new();
    let mut pipeline: Pipeline = Vec::new();
    let mut cmd = SimpleCmd::default();
    let mut run_if = RunIf::Always;

    let flush_cmd = |cmd: &mut SimpleCmd, pipeline: &mut Pipeline| {
        if !cmd.argv.is_empty() || cmd.stdin_file.is_some() {
            pipeline.push(std::mem::take(cmd));
        }
    };

    let flush_pipeline = |pipeline: &mut Pipeline, list: &mut CommandList, run_if: &mut RunIf, next: RunIf| {
        if !pipeline.is_empty() {
            list.push((std::mem::take(pipeline), std::mem::replace(run_if, next)));
        } else {
            *run_if = next;
        }
    };

    for token in tokens {
        match token {
            Token::Word(w) => cmd.argv.push(w),
            Token::Pipe => {
                flush_cmd(&mut cmd, &mut pipeline);
            }
            Token::Semicolon => {
                flush_cmd(&mut cmd, &mut pipeline);
                flush_pipeline(&mut pipeline, &mut list, &mut run_if, RunIf::Always);
            }
            Token::And => {
                flush_cmd(&mut cmd, &mut pipeline);
                flush_pipeline(&mut pipeline, &mut list, &mut run_if, RunIf::OnSuccess);
            }
            Token::Or => {
                flush_cmd(&mut cmd, &mut pipeline);
                flush_pipeline(&mut pipeline, &mut list, &mut run_if, RunIf::OnFailure);
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

    flush_cmd(&mut cmd, &mut pipeline);
    if !pipeline.is_empty() {
        list.push((pipeline, run_if));
    }
    list
}
