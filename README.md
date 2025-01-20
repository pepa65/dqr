<!--
[![Crate](https://img.shields.io/crates/v/quircs.svg?style=flat-square)](https://crates.io/crates/quircs)
[![Downloads](https://img.shields.io/crates/d/quircs.svg?style=flat-square)](https://crates.io/crates/quircs)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/quircs)
[![CI](https://github.com/dignifiedquire/quircs/workflows/CI/badge.svg)](https://github.com/dignifiedquire/quircs/actions)
-->
#rqr 0.10.3
**QR Scanner with Quircs**

* After: <https://github.com/dignifiedquire/quircs> which was ported from <https://github.com/dlbeer/quirc>
* 

## Install
### Download and install static single-binary
```
wget https://github.com/pepa65/rqr/releases/download/0.10.3/rqr
sudo mv rqr /usr/local/bin
sudo chown root:root /usr/local/bin/rqr
sudo chmod +x /usr/local/bin/rqr
```

## Install with cargo
If not installed yet, install a **Rust toolchain**, see <https://www.rust-lang.org/tools/install>

### Static build for Linux (avoiding GLIBC incompatibilities)
```
git clone https://github.com/pepa65/rqr
cd rqr
rustup target add x86_64-unknown-linux-musl
cargo rel  # Alias defined in .cargo/config.toml
```

The binary will be at `target/x86_64-unknown-linux-musl/release/rqr`

## Usage
`rqr <path-to-image>`

