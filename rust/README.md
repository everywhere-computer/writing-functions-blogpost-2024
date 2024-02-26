# Rust

For Rust, we implement an add function. The add function adds two signed integers and returns a signed integer result. It also logs the addition operation.

## Setup

Install the latest stable version of [Rust][install-rust].

Install [`cargo-component`][cargo-component]:

```sh
cargo install cargo-component
```

The initial code was generated with the `cargo component new --lib math` command, which generates Hello World starter code. We've adapted that to replace it with our add function.

We linked the WASI logging dependency from the `wit` directory in the root of our project:

```sh
cargo component add --target --path wit/deps/logging wasi:logging
```

This command adds bindings that make logging available through our Rust code:

```rust
use bindings::wasi::logging::logging::{log, Level};
```

## Build

Build for the debug target:

```sh
cargo component build
```

Build for release:

```sh
cargo component build --release
```

The builds target `wasm32-wasi` and are compiled to `rust/target/wasm32-wasi/debug/math.wasm` and `rust/target/wasm32-wasi/release/math.wasm` respectively.

[cargo-component]: https://github.com/bytecodealliance/cargo-component
[install-rust]: https://www.rust-lang.org/tools/install
