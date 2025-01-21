<!--
[![Crate](https://img.shields.io/crates/v/quircs.svg?style=flat-square)](https://crates.io/crates/sqr)
[![Downloads](https://img.shields.io/crates/d/quircs.svg?style=flat-square)](https://crates.io/crates/sqr)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/sqr)-->
[![Dependencies](https://deps.rs/repo/github/pepa65/sqr/status.svg)](https://deps.rs/repo/github/pepa65/sqr)
[![CI](https://github.com/pepa65/sqr/workflows/CI/badge.svg)](https://github.com/pepa65/sqr/actions)
[![License](https://img.shields.io/github/license/pepa65/sqr)](https://github.com/pepa65/sqr/blob/main/LICENSE)

# sqr 0.12.3
**Scan QR with Quircs**

* After: <https://github.com/dignifiedquire/quircs> which was ported from <https://github.com/dlbeer/quirc>
* Repo: <https://github.com/pepa65/sqr>

## Install
### Download and install static single-binary
```
wget https://github.com/pepa65/sqr/releases/download/0.12.3/sqr
sudo mv sqr /usr/local/bin
sudo chown root:root /usr/local/bin/sqr
sudo chmod +x /usr/local/bin/sqr
```

## Install with cargo
If not installed yet, install a **Rust toolchain**, see <https://www.rust-lang.org/tools/install>

### Direct from crates.io
`cargo install sqr --example sqr`

### Direct from repo
`cargo install --git https://github.com/pepa65/sqr --example sqr`

### Static build for Linux (avoiding GLIBC incompatibilities)
```
git clone https://github.com/pepa65/sqr
cd sqr
rustup target add x86_64-unknown-linux-musl
cargo rel  # Alias defined in .cargo/config.toml
```

The binary will be at `target/x86_64-unknown-linux-musl/release/sqr`

### Install with cargo-binstall
Even without a full Rust toolchain, rust binaries can be installed with the static binary `cargo-binstall`:

```
# Install cargo-binstall for Linux x86_64
# (Other versions are available at https://crates.io/crates/cargo-binstall)
wget github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
tar xf cargo-binstall-x86_64-unknown-linux-musl.tgz
sudo chown root:root cargo-binstall
sudo mv cargo-binstall /usr/local/bin/
```

Only a linux-x86_64 (musl) binary available: `cargo-binstall sqr`

It will be installed in `~/.cargo/bin/` which still needs to be added to `PATH`!

## Usage
```
sqr 0.12.3
Usage:  sqr [-h|--help] | [-v|--verbose] [-d|--dump] <image>...
    -h/--help       Show this help text
    -v/--verbose    Show processing information
    -d/--dump       Dump each identified QR code to the terminal
```
