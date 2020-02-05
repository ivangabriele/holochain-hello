# Hello Holochain

- [Hello Holochain](#hello-holochain)
  - [Getting started](#getting-started)
    - [Install Rust](#install-rust)
    - [Install Nix](#install-nix)
      - [Fix known issue under macOS Catalina](#fix-known-issue-under-macos-catalina)
    - [Install Holochain](#install-holochain)
    - [Configure Visual Studio Code](#configure-visual-studio-code)
    - [Compile](#compile)
    - [Run locally](#run-locally)
    - [Check](#check)

## Getting started

### Install Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup update
# Install racer:
# https://github.com/racer-rust/racer
# which is used by `kalitaalexey.vscode-rust` VS Code extension
# with this known issue: https://stackoverflow.com/a/53137160/2736233
rustup install nightly
cargo +nightly install racer
# Install RLS:
# https://github.com/rust-lang/rls
# which is used by `kalitaalexey.vscode-rust` VS Code extension
rustup component add rls rust-analysis rust-src
```

### Install Nix

```sh
curl https://nixos.org/nix/install | sh
```

#### Fix known issue under macOS Catalina

**First installation:**

Following [this known issue](https://github.com/NixOS/nix/issues/2925#issuecomment-539570232):

1. Run:
   ```sh
   echo 'nix' | sudo tee -a /etc/synthetic.conf
   ```
2. Reboot
3. Run:
   ```sh
   sudo diskutil apfs addVolume disk1 APFSX Nix -mountpoint /nix
   sudo diskutil enableOwnership /nix
   sudo chflags hidden /nix
   echo "LABEL=Nix /nix apfs rw" | sudo tee -a /etc/fstab
   curl https://nixos.org/nix/install | sh
   ```

**After each reboot:**

> **[TODO] Double-check that.**

Following [this related issue](https://github.com/NixOS/nix/issues/3125), after each reboot, run:

```sh
launchctl load /Library/LaunchDaemons/org.nixos.nix-daemon.plist && launchctl start org.nixos.nix-daemon
```

### Install Holochain

```sh
nix-shell https://holochain.love
```

### Configure Visual Studio Code

1. Install [nixfmt](https://github.com/serokell/nixfmt)
2. Open `holochain-hello` directory in VS Code
3. Install the recommanded extensions
4. Reload VS Code
5. Open the commands pallet (<kbd>CMD</kbd>/<kbd>CTRL</kbd>+<kbd>SHIFT</kbd>+<kbd>P</kbd>), find
   `Nix-Env: Select environment` and pick `default.nix` in the list.

### Compile

```sh
nix-shell https://holochain.love
hc package
```

### Run locally

In the same session:

```sh
hc run -i http
```

### Check

In another session:

```sh
curl -X POST -H "Content-Type: application/json" -d '{ "id": "0", "jsonrpc": "2.0", "method": "call", "params": { "instance_id": "test-instance", "zome": "hello", "function": "hello", "args": {} } }' http://127.0.0.1:8888 | node <<< "const o = $(cat); console.log(JSON.stringify(o, null, 2));"
```

should return:

```json
{
  "jsonrpc": "2.0",
  "result": "{\"Ok\":\"Hello Holochain!\"}",
  "id": "0"
}
```
