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

## install

**linux (no rust needed)**
```
curl -L https://github.com/yourname/rsh/releases/latest/download/rsh-linux-x86_64 -o rsh && chmod +x rsh && sudo mv rsh /usr/local/bin/rsh
```

**mac (apple silicon)**
```
curl -L https://github.com/yourname/rsh/releases/latest/download/rsh-macos-arm64 -o rsh && chmod +x rsh && sudo mv rsh /usr/local/bin/rsh
```

**from source**
```
git clone https://github.com/yourname/rsh && cd rsh && cargo install --path .
```

then just type `rsh`

## running locally

```
cargo run
```

## prompt

shows current directory and git branch if in a repo

```
~ shell git:(main) $
```
