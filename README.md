# shell

a basic unix shell written in rust for learning purposes

## features

- run external commands
- pipes (`cmd1 | cmd2`)
- i/o redirection (`>`, `>>`, `<`)
- command history (arrow keys)
- tab completion
- `$VAR` expansion, `~` expansion, quote handling
- builtins: `cd`, `echo`, `pwd`, `clear`, `exit`, `export`, `unset`, `env`, `which`, `type`

## running

```
cargo run
```

## prompt

shows current directory and git branch if in a repo

```
~ shell git:(main) $
```
