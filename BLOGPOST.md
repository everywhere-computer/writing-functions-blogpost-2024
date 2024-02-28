Everywhere Computer is a compute platform for InterPlanetary Virtual Machine (IPVM) workflows. The Homestar runtime implements IPVM and runs compute in the Everywhere Computer.

Homestar runs Wasm-based workflows where Wasm components provide functions to execute. Wasm components can be authored in Rust, JavaScript, and Python. Reading ahead, we'll be writing functions in each of these languages, compiling them to Wasm, packaging them as Wasm components, and bringing them together into a workflow.

Our goal is to introduce authoring functions for Everywhere Computer. Along the way, we'll introduce Wasm component tooling, the Homestar runtime, and the `every-cli` that runs the Homestar runtime and a gateway for managing Wasm components and preparing workflows.

### Wasm components, WIT, and WASI logging

#### Wasm Components

- Why?
- Bytecodealliance work on components
- We'll use their tooling for writing components
- We also use their Wasmtime in Homestar!

#### WIT

- Describe Wasm component interfaces
- Consistent interface implemented by our source languages
- Has it's own type system

#### WASI logging

- Introduce WASI
- Homestar acts as a host that implements WASI logging
- Our functions log messages that are displayed by Homestar
- Can use for logging information or reporting errors

### Our functions

We will write arithmetic operations in each language to keep our example code simple. However, we are daring enough to include division to introduce division by zero errors and floating point numbers.

Rust will perform addition and division, JavaScript will perform subtraction, and Python multiplication.

#### Rust

- Explain cargo component
- Show testing functions with wasmtime
- Details...

#### JavaScript

- Why Wasm components so big? Explain Spidermonkey
- Why is it so slow? Explain Spidermonkey
- Details...

#### Python

- Words...

### IPFS

Homestar and Everywhere Computer use IPFS as a storage layer. Before we start into the next section, [install IPFS Kubo][install-ipfs] and start the IPFS daemon:

```sh
ipfs dameon
```

The daemon should run on the default `5001` port.

### Workflows

We now have a set of Wasm components with arithmetic functions sourced from multiple languages. Our next step is to run these functions together in workflows.

The `every-cli` starts a gateway that loads Wasm components, prepares workflows, and calls on the Homestar runtime to execute them. [Install `every-cli`][install-every-cli], and we'll write a workflow.

The workflows that Homestar runs are a bit challenging to write by hand, so `every-cli` provides a simplfied workflow syntax that it uses to prepare the underlying workflow. Let's start by using `math.wasm` to add two numbers:

```json
{
  "tasks": [
    {
      "run": {
        "name": "add",
        "input": {
          "args": [3.1, 5.2],
          "func": "add"
        }
      }
    }
  ]
}
```

A workflow is an array of tasks that we would like to execute. Each task is given a `name` which will be used to reference results in subsequent tasks. Our task `input` includes the name of the function to execute and the arguments to the function.

Let's run this workflow! Start `every-cli` with `math.wasm` as argument:

```sh
every dev --fn rust/target/wasm32-wasi/release/math.wasm
```

`every-cli` will start a gateway that we can query for a JSON Schema representing the WIT interfaces in `math.wasm` at `localhost:3000`.

Post the workflow to the gateway:

```sh
curl localhost:3000/run --json @workflows/add.json
```

The response should contain the result of adding `3.1` and `5.2`:

```sh TODO: Replace with screenshot
» curl localhost:3000/run --json @workflows/add.json
8.299999
```

In addition, `every-cli` has passed along our WASI log from the Homestar runtime:

```
ts=2024-02-28T00:19:22.413842Z level=info target=homestar_wasm::wasmtime::host::helpers message="3.1 + 5.2 = 8.299999" subject=wasm_execution category=guest:rust:add
```

- All four functions
  - Run workflow with all four functions (`workflows/all.json`)
  - Show logs and note execution time for each
- Show logs with a division by zero error
  - Run workflow with division by zero (`workflows/division_by_zero.json`)

### Everywhere Computer Control Panel

You may have noticed `every-cli` starts a Control Panel:

```

✔ Control Panel is running at http://127.0.0.1:4173

```

We have a web UI in progress that we will discuss in a future post.

[install-every-cli]: https://www.npmjs.com/package/@everywhere-computer/every-cli
[install-ipfs]: https://docs.ipfs.tech/install/command-line/#install-official-binary-distributions
