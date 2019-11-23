# evu

Implementation of ARQ data format and command line tool to restore backups

Note: there is no caching in this early state of the tool therefore all commands are much
more inneficient than they should be.

## Build

```rust
cargo build --release
```

## Usage

`evu` works only with local paths. If you have your backed up folders with Arq locally,
make sure you pass the top folder path to `-p`.

```rust
$ evu -h
evu 0.1.0
Norberto Lopes <nlopes.ml@gmail.com>
Command line interface to ARQ

USAGE:
    evu [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --path <path>    Path to the backup folder

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    restore    restore file
    show       display one or more resources
```

There are 2 commands to be aware of:

- show
  - computers
  - folders
  - tree
- restore
