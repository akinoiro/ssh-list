# SSH-List
SSH connection manager with a TUI interface.

![demo gif](https://raw.githubusercontent.com/akinoiro/ssh-list/main/images/demo.gif)

Key Features:
- Add and edit connections
- Rearrange and sort connections
- Import hosts from ~/.ssh/config
- Search and filter connections
- Execute commands on remote hosts

This application does not modify your existing SSH configuration files. Host settings can be spread across multiple files referenced by Include directives (and between system and user configs), so automatic editing is unreliable.

This application does not store passwords. For secure authentication, use SSH keys.

## Prerequisites
You must have an OpenSSH client installed on your system.
## Install from GitHub Release
Download the latest binary from the [Releases page](https://github.com/akinoiro/ssh-list/releases).

#### To run the `ssh-list` command from terminal:
```
# Make the binary executable
chmod +x ssh-list
# Move it to a directory in your PATH
sudo mv ssh-list /usr/local/bin/
```

## Install from crates.io
```
cargo install ssh-list
```
## Install from AUR (Arch Linux)
```
paru -S ssh-list
```
## Install from PPA (Ubuntu, Linux Mint)
```
sudo add-apt-repository ppa:akinoiro/ssh-list
sudo apt update
sudo apt install ssh-list
```
## Install from Homebrew (macOS, Linux)
```
brew tap akinoiro/tap
brew install ssh-list
```
## Build from source
You will need Rust and Cargo installed.
```
git clone https://github.com/akinoiro/ssh-list
cd ssh-list
cargo build --release
```
The binary will be located at target/release/
## Configuration files
ssh-list automatically creates files to store your connections:
```
~/.ssh/ssh-list.json
```
and application settings:
```
~/.ssh/ssh-list_config.toml
```
## Appearance customization

![demo settingsgif](https://raw.githubusercontent.com/akinoiro/ssh-list/main/images/demo_settings.gif)
