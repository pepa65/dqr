[![Crate](https://img.shields.io/crates/v/dqr.svg?style=flat-square)](https://crates.io/crates/dqr)
[![Downloads](https://img.shields.io/crates/d/dqr.svg?style=flat-square)](https://crates.io/crates/dqr)
[![Docs](https://img.shields.io/badge/docs-dqr-blue.svg?style=flat-square)](https://docs.rs/crate/dqr/latest)
[![Dependencies](https://deps.rs/repo/github/pepa65/dqr/status.svg)](https://deps.rs/repo/github/pepa65/dqr)
[![CI](https://github.com/pepa65/dqr/workflows/CI/badge.svg)](https://github.com/pepa65/dqr/actions)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/pepa65/dqr/blob/master/LICENSE)

# dqr 0.14.5
**Decode QR with Quircs**

* After: <https://github.com/dignifiedquire/quircs> which was ported from <https://github.com/dlbeer/quirc>
* Repo: <https://github.com/pepa65/dqr>

## Install
### Download and install static single-binary
```
wget https://github.com/pepa65/dqr/releases/download/0.14.5/dqr
sudo mv dqr /usr/local/bin
sudo chown root:root /usr/local/bin/dqr
sudo chmod +x /usr/local/bin/dqr
```

## Install with cargo
If not installed yet, install a **Rust toolchain**, see <https://www.rust-lang.org/tools/install>

### Direct from crates.io
`cargo install dqr --example dqr`

The binary `dqr` will be installed into `~/.cargo/bin/` (might need to be added to `PATH`!)

### Direct from repo
`cargo install --git https://github.com/pepa65/dqr --example dqr`

The binary `dqr` will be installed into `~/.cargo/bin/` (might need to be added to `PATH`!)

### Static build for Linux (avoiding GLIBC incompatibilities)
```
git clone https://github.com/pepa65/dqr
cd dqr
rustup target add x86_64-unknown-linux-musl
cargo rel  # Alias defined in .cargo/config.toml
```

The binary will be at `target/x86_64-unknown-linux-musl/release/dqr`

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

Then a linux-x86_64 (musl) binary can be installed: `cargo-binstall dqr`

The binary `dqr` will be installed into `~/.cargo/bin/` (might need to be added to `PATH`!)

## Usage
```
dqr 0.14.5
Usage:  dqr [-h|--help] | [-v|--verbose] [-d|--dump] <image>...
    -h/--help       Show this help text
    -v/--verbose    Show additional information on each QR
    -d/--dump       Dump each identified QR code to the terminal
```
