# rustfmt-shim

A temporary workaround for a few intellij-rust issues:

- rustfmt toolchain
  - [#6061 Feature request: Run nightly rustfmt on save](https://github.com/intellij-rust/intellij-rust/issues/6061)
- auto-import woes
  - [#5654 `extern` exports of std suggested before std](https://github.com/intellij-rust/intellij-rust/issues/5654)
  - [#5997 Auto import picks private module](https://github.com/intellij-rust/intellij-rust/issues/5997)
  - [#6399 Rust plugin keeps using "use futures_core::core_reexport::xxx" insted of use "std::"](https://github.com/intellij-rust/intellij-rust/issues/6399)

The workaround is to replace the default `rustfmt` with a shim, which tricks intellij-rust into running it instead of rustfmt.

Here is what the shim does:

- Preprocesses the source to fix imports
- Runs a different toolchain's rustfmt, if one is found in `.pre-commit-config.yaml`
  - If no override is found, falls back to stable rustfmt

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
