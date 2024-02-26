Everywhere Computer is a compute platform for InterPlanetary Virtual Machine (IPVM) workflows. The Homestar runtime implements IPVM and runs compute in the Everywhere Computer.

Homestar runs Wasm-based workflows where Wasm components provide functions to execute. Wasm components can be authored in Rust, JavaScript, and Python. Reading ahead, we'll be writing functions in each of these languages, compiling them to Wasm, packaging them as Wasm components, and bringing them together into a workflow.

Our goal is to introduce authoring functions for Everywhere Computer. Along the way, we'll introduce Wasm component tooling, the Homestar runtime and CLI, and the Everywhere Computer Control Panel.

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

### Homestar

Introduce Homestar.

#### Workflows

- Add only
  - Show running only the addition function
  - Show logging and retrieving receipt from IPFS
  - Show where the instruction CID
- Add and subtract
  - Add JavaScript subtract to the worklflow, awaiting result of addition
  - Run that and show logs again
- All four functions
  - Show logs and note execution time for each
  - Show a second run with replays
  - Division by zero
  - Update workflow to divide by zero at end
- Show logs with a division by zero error

### Everywhere Computer

- Show workflow construction in Control Panel
- Run it, screenshots, etc
