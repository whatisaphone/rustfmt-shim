# rustfmt-shim

A workaround for this intellij-rust bug:

>[Support use of rustfmt-nightly](https://github.com/intellij-rust/intellij-rust/issues/1343)

The workaround is to replace the default `rustfmt` with a shim that:

- Determines the desired toolchain by parsing `.pre-commit-config.yaml`
- Runs that toolchain's rustfmt instead

Thus, we can trick intellij-rust into running any toolchain's rustfmt.

## Usage

- Clone the source
- Run `cargo run --release -- --install-the-shim`
  - This backs up, and then replaces, `~/.cargo/bin/rustfmt`
- In IntelliJ, enable "Run rustfmt on Save"

If it doesn't work, check `/tmp/rustfmt-shim.log` for hints. If it does work, the target directory and source tree are no longer needed, so you can delete them if you wish.

Note: If the shim is ever clobbered by e.g. rustup, you'll need to install again. Shimfight!

## Development

### Install prerequisites

- [Rust]
- [pre-commit]

[Rust]: https://www.rust-lang.org/
[pre-commit]: https://pre-commit.com/

### Install the pre-commit hook

```sh
pre-commit install
```

This installs a Git hook that runs a quick sanity check before every commit.

### Run the app

```sh
cargo run
```

### Run the tests

```sh
cargo test
```
