# anyversion

**You don't need try hard to get version number anymore.**

## What's anyversion?

The hardest problem in this galaxy is "How to get the version of the installed command".

There are four popular ways to show a version.

```
ffmpeg -version
node --version
python -V
rustc version
```

`anyversion` makes these into single syntax.

```
anyversion ffmpeg
anyversion node
anyversion python
anyversion rustc
```

That's all.

## Install

Install rust toolchain with `rustup` from below URL.

https://rustup.rs/

Next, use `cargo install` (which is installed by `rustup`) to install `anyversion`.

```
cargo install anyversion
```

## Update

Run `cargo install anyversion` again.

## Usage

```
# Syntax
anyversion <command_name>

# For example
anyversion bash
anyversion git
anyversion rustc
anyversion node
anyversion make
anyversion anyversion
```

## FAQ

### Do you have a plan to support <command_what_you_love>?

I'll try to support more commands. But it's open source software. You can contribute it. I think that writing code is more exciting than just waiting.

### Why do you use a whitelist? It's better to try to execute every args pattern.

It's safer. Some commands run unexpected process with invalid args. For example, `blahblah version` may make a project with `version` directory. `bazbaz -v` may start a REPL with verbose mode.