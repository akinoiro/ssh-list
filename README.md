# SSH-List
![demo gif](https://raw.githubusercontent.com/akinoiro/ssh-list/main/images/demo.gif)

SSH connection manager with a TUI interface.

Key Features:
- Add, edit, and delete connections.
- Sort your list of hosts.
- Specify options for each connection.
- This application does not store passwords. For secure authentication, use SSH keys.

## Prerequisites
You must have an OpenSSH client installed on your system.
## Install from GitHub Release
Download the latest binary from the [Releases page](https://github.com/akinoiro/ssh-list/releases).

#### To run the `ssh-list` command from any terminal:

Linux:
```
# Make the binary executable
chmod +x ssh-list
# Move it to a directory in your PATH
sudo mv ssh-list /usr/local/bin/
```

macOS:
```
# Manually grant permission to run the binary
xattr -d com.apple.quarantine ssh-list
# Make the binary executable
chmod +x ssh-list
# Move it to a directory in your PATH
sudo mv ssh-list /usr/local/bin/
```

Windows:
1.  Download and extract the `.zip` archive.
2.  Move `ssh-list.exe` to a permanent folder (e.g., `C:\Program Files\ssh-list`).
3.  Add that folder's location to your system's `PATH` environment variable.

## Install from crates.io
```
cargo install ssh-list
```
## Build from source
You will need Rust and Cargo installed.
```
git clone https://github.com/akinoiro/ssh-list
cd ssh-list
cargo build --release
```
The binary will be located at target/release/
## Configuration file
On the first run, ssh-list will automatically create a configuration file to store your connections.
```
~/.ssh/ssh-list.json
```
