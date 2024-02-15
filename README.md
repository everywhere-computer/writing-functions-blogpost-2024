# writing-functions-blogpost-2024

TODO: Intro

## Setup

Install or download a binary for the `wit-deps-cli`. Binaries are available on the [`wits-deps` releases page][wit-deps-releases] or Rust users can install it with `cargo`:

```sh
cargo install wit-deps-cli
```

## WIT Dependecies

Logging from our Wasm components depends on the propsed [WASI logging][wasi-logging] implementation. The dependency is listed in the [WIT dependencies manifest][wit-manifest], and we've included it our project by running the `wit-deps-cli`:

```sh
wit-deps
```

[wit-deps-cli]: https://github.com/bytecodealliance/wit-deps
[wit-deps-releases]: https://github.com/bytecodealliance/wit-deps/releases
[wasi-logging]: https://github.com/WebAssembly/wasi-logging
[wit-manifest]: ./wit/deps.toml
