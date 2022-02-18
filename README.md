# Rust Example Projects

## install rustup

```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
# rustup doc
```

## update rustup

```shell
rustup update
```

## uninstall rustup

```shell
rustup self uninstall
```

## create a new binary project

```shell
cargo new rs00x --bin --vcs none
```

## create a new library project

```shell
cargo new rs00x --lib --vcs none
cargo test -- --nocapture
```

## init a project

```shell
# cd rs00x
cargo init --bin --vcs none
```
## build and run a project

```shell
# cd rs00x
cargo update # taking effect after removed Cargo.lock
cargo check

rustup main.rs
cargo build # --release
cargo run # --release
```