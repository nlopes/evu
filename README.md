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
```

There are 2 commands to be aware of:

- show
  - computers
  - folders
  - tree
- restore
