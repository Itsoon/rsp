# RSP 🦀

> [!WARNING]
> RSP is currently only available on Linux with the Hyprland window manager and is compatible only with the kitty terminal.

# ⚡What is RSP ?

> RSP, (Rust Starter Profiles), is a CLI tool written in Rust. Its aim is to simplify the management and launching of different work profiles, according to the user's needs. This simplifies the process of configuring the work or relaxation environment in a single command.

# 🔥Features

- Configuration profile management

- Launching profiles from the Terminal

- Open applications in specific workspaces

# ⚙️Installation

> [!IMPORTANT]  
> Rust must be installed on your system.

Clone the repository :

```shell
git clone https://github.com/Itsoon/rsp.git; cd rsp
```

## Automated installation (Setup)

Run Setup :

```shell
./setup.sh
```

Installation check :

```shell
rsp --version
```

## Manual installation

Compiling :

```shell
cargo build --release
```

Copy the binary :

```shell
sudo cp ./target/release/rsp /usr/local/bin/
```

Installation check :

```shell
rsp --version
```
