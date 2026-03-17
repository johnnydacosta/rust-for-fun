# Rust for fun

## Setup

### Installing rust

Via curl: 

```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
Or via Homebrew

```shell
brew install rust
```

### Upgrade

```shell
rustup update 
```

### Uninstall

```shell
rustup self uninstall
```

### check Rust version

```shell
rustc --version
```
### create a new project


Create a new project with directory and package name app-name

```shell
cargo new app-name
```
app-name/
├── Cargo.toml   # project configuration
└── src/
    └── main.rs  # main Rust file


Create a new project with directory app-01 and package name app-name

```shell
cargo new --name app-name app-01
```

app-01/
├── Cargo.toml   # project configuration
└── src/
    └── main.rs  # main Rust file
