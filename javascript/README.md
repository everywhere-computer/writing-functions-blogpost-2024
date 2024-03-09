# JavaScript

For JavaScript, we implement a subtract function. The subtract function subtracts one float from another and returns a float result. It also logs the subtraction operation.

This function is componentized using [Homestar Wasmify][homestar-client].

## Setup

Install the latest version of [Node][install-node]. 

Install dependencies:

```sh
npm install
```

## Build

Build using the provided node script:

```sh
node index.js
```

The node script calls Wasmify's `build` function with `src/subtract.ts` as an input and emits a Wasm component to the `output` directory.

Wasmify infers WIT types from TypeScript. In our function, `number` is inferred to be WIT `float64`.

Note that Wasmify does not require a WIT file and will automatically include the WASI logging dependency imported by our function.

[homestar-client]: https://www.npmjs.com/package/@fission-codes/homestar
[install-node]: https://nodejs.org/en/download
