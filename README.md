# writing-functions-blogpost-2024

This repository contains the example code from the [Writing Functions for Everywhere Computer][published-post] blog post. The contents of the post are also reproduced in [BLOGPOST.md][local-blogpost].

Wasm components are documeted individually by their source language:

- [Rust][rust-component]: A math component that implements addition and division with division by zero error reporting
- [JavaScript][javascript-component]: A component that implements subtraction
- [Python][python-component]: A component that implements multiplication

## Setup

Install or download a binary for the `wit-deps-cli`. Binaries are available on the [`wits-deps` releases page][wit-deps-releases] or Rust users can install it with `cargo`:

```sh
cargo install wit-deps-cli
```

## WIT Dependecies

Logging from our Wasm components depends on the propsed [WASI logging][wasi-logging] implementation. This dependency is listed in the [WIT dependencies manifest][wit-manifest], and we've included it our project by running the `wit-deps-cli`:

```sh
wit-deps
```

[javascript-component]: ./javascript
[local-blogpost]: ./BLOGPOST.md
[published-post]: https:://TODO-LINK-TO-PUBLISHED
[python-component]: ./python
[rust-component]: ./rust
[wit-deps-cli]: https://github.com/bytecodealliance/wit-deps
[wit-deps-releases]: https://github.com/bytecodealliance/wit-deps/releases
[wasi-logging]: https://github.com/WebAssembly/wasi-logging
[wit-manifest]: ./wit/deps.toml
