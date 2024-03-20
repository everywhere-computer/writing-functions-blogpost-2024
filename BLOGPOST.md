# Writing Functions for Everywhere Computer

[Everywhere Computer][everywhere-comp] is an emerging decentralized platform that aims to distribute computational tasks across a vast, open network. This network spans from your personal machine to other devices on your LAN, a cluster of cloud nodes, and even to [PoPs (point of presences)][pop] located at the edge of the Internet. Processing happens as close to the data source as possible or on machines with general availability or critical resources where relocating data is worthwhile.

At its core, Everywhere Computer is built on the [InterPlanetary Virtual Machine (IPVM)][ipvm] protocol. It executes [workflows][workflows] containing tasks that are [content-addressed][content-addressing]—which means they're uniquely identified by their content rather than by their location. This system is powered by our [Homestar runtime][homestar-runtime], an engine that runs Wasm-based workflows composed of [Wasm components][wasm-component] with runnable functions that can be scheduled and executed by any Homestar peer throughout the network.

![everywhere-at-a-glance](./assets/blogcompute.png)

Beyond the sandboxing, portability, and predictable performance benefits of Wasm, we're excited about orchestrating workflows and state machines composed of modules compiled from different source languages. With Everywhere Computer, we're all in on "[the return of write once, run anywhere][write-once-run]", but with content-addressing and our focus on replayability of previously computed tasks, we can go a step further and say "**write once, run once, and then never again, everywhere**."

With this post, our goal is to introduce authoring Wasm components and, therefore, functions for Everywhere Computer. Wasm components can be authored in [various languages][wit-guest][^1], but we'll focus primarily on Rust, JavaScript, and Python for this post. Reading ahead, we'll be writing functions in each of these languages, compiling them to Wasm, packaging them as Wasm components, and bringing them together into a workflow that executes on our compute platform. Along the way, we'll introduce Wasm component tooling, the Homestar runtime, and [Every CLI][everycli], the latter of which provides a convenient interface for running Homestar with a gateway for preparing and executing workflows.

Everywhere Computer is in beta. Everything is publicly available, but we have a closed beta group to provide high-quality support and to gather feedback. [Sign up][beta-signup] for the beta group. We would love to hear what you are working on and how you might use Everywhere Computer!

The code covered in this post is available in the [writing-functions-blogpost-2024][writing-functions-repo] repository.

### Wasm components, WIT, and WASI logging

Evolution within the Wasm ecosystem is happening at a wicked fast pace, particularly now that the [path to Wasm components][path-to-components] has been streamlined and standardized, module-to-module interop is trivial.

In Everywhere Computer, we decided to use the [Canonical ABI][canonical-abi] for converting between the values and functions of Component Model components with those of [Core WebAssembly][core-wasm] modules instead of imposing a custom ABI upon our users. A component is just a wrapper around a core module that specifies its imports, internal definitions, and exports using interfaces defined by the [WIT][wit] IDL format.

Unlike core modules, components may not export Wasm memory, reinforcing Wasm sandboxing and enabling interoperation between languages with different memory assumptions. For example, a component that relies on Wasm-GC (garbage collected) memory compiled from a dynamic language can seamlessly interact with a component compiled from a static language using linear memory.

Everywhere Computer strives for [simplicity][simple-made-easy]. By adopting the Component model and its tooling (for example, [cargo-component][cargo-component] and [wit-bindgen][wit-bindgen]), we can run workflows combining components from different languages without handling incomplete Wasm modules or introducing custom tooling, bindgens, or SDKs for our ecosystem.

In addition, our Homestar runtime utilizes alternate formats as internal [intermediate representations][ir]. By adopting WIT, we can [interpret][wit-to-ipld] between WIT values and other data models at runtime without function writers knowing anything about our internal formats.

#### Embedding Wasmtime

The Homestar runtime embeds the [Wasmtime][wasmtime] runtime to execute Wasm components (outside of the web) associated with tasks in a workflow. Built and maintained by the [Bytecode Alliance][bytecode-alliance], the Wasmtime runtime delivers on driving multi-language support and fine-grained configuration for things like CPU and memory usage. With platforms and frameworks like [wasmCloud][wasmcloud], [Spin][fermyon-spin], and [Fastly Compute][fastly-compute] all integrating with Wasmtime for their endeavors, we're in good company. Plus, we needed a runtime that's at the forefront of the latest developments in the Wasm ecosystem, including the WASI stack, which recently reached [WASI Preview 2][wasip2], enabling library developers and implementers, like ourselves, the ability to work with and build upon lower level primitives from a stable set of common interfaces.

#### Wit

#### WASI Logging

<!-- #### Wasm Components -->

<!-- - Why? -->
<!-- - Bytecodealliance work on components -->
<!-- - We'll use their tooling for writing components -->
<!-- - We also use their Wasmtime in Homestar! -->

<!-- #### WIT -->

<!-- - Describe Wasm component interfaces -->
<!-- - Consistent interface implemented by our source languages -->
<!-- - Has it's own type system -->

<!-- #### WASI logging -->

<!-- - Introduce WASI -->
<!-- - Homestar acts as a host that implements WASI logging -->
<!-- - Our functions log messages that are displayed by Homestar -->
<!-- - Can use for logging information or reporting errors -->

### Our functions

We will write arithmetic operations in each language to keep our example code simple and straightforward. We will use division to show division by zero error reporting.

Our Rust program will perform addition and division; the JavaScript one will perform subtraction; and, the Python program will carry out multiplication.

Our functions will be compiled to Wasm components using tools from or built upon the excellent work from the [Bytecode Alliance][bytecode-alliance]. The Wasm component ecosystem is evolving quickly, so keep in mind that the techniques described in this blog post may be out of date. We'll provide links so you can check on the latest developments.

#### Rust

For Rust, we use [`cargo component`][cargo-component] to generate a Wasm component. `cargo component` imagines what first-class support for WebAssembly components might look like for Rust.

Rust support includes referencing WIT dependencies in the Cargo manifest. We reference WASI logging in our manifest:

```toml
[package.metadata.component.target.dependencies]
"wasi:logging" = { path = "../wit/deps/logging" }
```

We set our target WIT world in the manifest as well:

```toml
[package.metadata.component.target]
path = "../wit/math.wit"
world = "math"
```

Our WIT interface defines `add` and `divide` functions:

```
package fission:math@0.1.0;

world math {
  import wasi:logging/logging;

  export add: func(a: float64, b: float64) -> float64;
  export divide: func(a: float64, b: float64) -> float64;
}
```

`cargo component` generates a set of bindings that produce a `Guest` trait that requires us to implement the interfaces from our WIT world. It also provides an interface for the WASI logging dependency.

Our Rust source code implements `add` and `divide` with logging for each operation and error reporting when division by zero would occur.

```rust
#[allow(warnings)]
mod bindings;

use bindings::wasi::logging::logging::{log, Level};
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn add(a: f64, b: f64) -> f64 {
        let result = a + b;

        log(
            Level::Info,
            "guest:rust:add",
            format!("{a} + {b} = {result}").as_str(),
        );

        result
    }

    fn divide(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            log(
                Level::Error,
                "guest:rust:divide",
                format!("Division by zero error").as_str(),
            );

            panic!()
        }

        let result = a / b;

        log(
            Level::Info,
            "guest:rust:divide",
            format!("{a} / {b} = {result}").as_str(),
        );

        result
    }
}

bindings::export!(Component with_types_in bindings);
```

`cargo component build` generates the necessary bindings and outputs a `math.wasm` component to the `target/wasm32-wasi/debug` directory. A `cargo component build --release` build outputs to `target/wasm32-wasi/release`.

#### JavaScript

For JavaScript, we use [Homestar Wasmify][homestar-client] to generate a Wasm component. Wasmify is our tool to generate Wasm components from JavaScript code. Wasmify generates Wasm components by bundling JavaScript code, generating WIT types from TypeScript code or JSDoc defined types, and embedding WASI dependencies. Keep in mind that Wasmify is in development and does not support all WIT defined types.

To generate a Wasm component Wasmify will bundle the JS code, generate WIT types from TypeScript code or JSDoc defined types and embed WASI dependencies

Our TypeScript source code subtracts two numbers and logs the operation:

```typescript
import { log } from "wasi:logging/logging";

export function subtract(a: number, b: number): number {
  const result = a - b;

  log("info", "guest:javascript:subtract", `${a} - ${b} = ${result}`);

  return result;
}
```

Building a Wasm component from this source code calls Wasmify `build`:

```javascript
import { build } from "@fission-codes/homestar/wasmify";

await build({
  entryPoint: "src/subtract.ts",
  outDir: "output",
});
```

Running this script will produce a Wasm component with a `subtract` name prefix and a hash, for example `subtract-j54di3rspj2eewjro4.wasm`.

Wasmify is built on top of [ComponentizeJS][componentize-js] which ingests JavaScript source code and embeds SpiderMonkey in a Wasm component to run it. Embedding SpiderMonkey and running JavaScript code comes at a size and perfomance cost compared to languages that can compile to WebAssembly directly, but it is necessary to provide a JavaScript environment.

See [Making JavaScript run fast on WebAssembly][javascript-webassembly-post] for more information.

#### Python

For Python, we use [componentize-py][componentize-py] to generate a Wasm component.

Our WIT interface defines a `multiply` function:

```
package fission:math@0.1.0;

world multiplication {
  import wasi:logging/logging;

  export multiply: func(a: float64, b: float64) -> float64;
}
```

`componentize-py` generates a set of bindings to import in our Python source code. Unlike Rust, the bindings do not need to be written to a file and can be generated on the fly.

Our Python source code multiplies two numbers and logs the operation:

```python
import multiplication
from multiplication.imports.logging import (log, Level)

class Multiplication(multiplication.Multiplication):
    def multiply(self, a, b) -> float:
        result = a * b

        log(Level.INFO, 'guest:python:multiply', '{} * {} = {}'.format(a, b, result))

        return a * b
```

We run `componentize-py` to generate our Wasm component:

```sh
componentize-py -d ../wit -w multiplication componentize app -o output/multiply.wasm
```

The `-d` option tells `componentize-py` where to look for our WIT interfaces and `-w` tells it which WIT world to use. The `componentize` command takes the name of the Python module containing the app to wrap. In our case, we are targeting `app.py`.

`componentize-py` bundles `CPython`, `libc` and other dependencies into the Wasm component to interpret and provide a Python environment for our code. Like JavaScript, this comes at a size and performance cost but is necessary to run Python code.

We recommend reading the [Introducing Componentize-Py][introducing-componentize-py-blog] blog post for more information on writing Python sourced components.
Also, the [Introducing Componentize-Py: A Tool for Packaging Python Apps as Components][introducing-componentize-py-video] is an excellent talk that explains how `componentize-py` works.

### IPFS

Homestar and Everywhere Computer currently uses [IPFS][ipfs] as a storage layer. Before we start into the next section, [install IPFS Kubo][install-ipfs] and start the IPFS daemon:

```sh
ipfs daemon
```

The daemon should run on the default `5001` port.

### Workflows

We now have a set of Wasm components with arithmetic functions sourced from multiple languages. Our next step is to run these functions in [workflows][workflows].

Every CLI starts a gateway that loads Wasm components onto IPFS, prepares workflows, and calls on the Homestar runtime to schedule and execute them. [Install Every CLI][install-every-cli], then we'll write a workflow.

The workflows that Homestar runs are a bit challenging to write by hand directly, so Every CLI provides a simplfied workflow syntax that it uses to prepare the underlying workflow. Let's start by using `math.wasm` to add two numbers:

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

Let's run this workflow! Start Every CLI with `math.wasm` as an argument:

```sh
every dev --fn rust/target/wasm32-wasi/release/math.wasm
```

Every CLI starts a gateway that we can query for a JSON Schema representing the WIT interfaces in `math.wasm` at `localhost:3000`.

Post the workflow to the gateway:

```sh
curl localhost:3000/run --json @workflows/add.json
```

The response reports the result of adding `3.1` and `5.2` as `8.3`.

In addition, Every CLI has passed along logs from the Homestar runtime:

![add-logs](assets/add.png)

The logs report information about workflow execution and include our WASI logs. Our WASI log reports `"3.1 + 5.2 = 8.3"` with the category `guest:rust:add`. WASI logs always have the `wasm_execution` subject.

We can also see workflow settings, fetching resources (our Wasm components), intializing, starting, and completing the workflow. The resolving receipts log shows that Homestar is looking for cached results so it can avoid work where possible. The computed receipt log reports the [CID][cid], a content identifier derived on the content's cryptographic hash and which points to material on IPFS, of the receipt from the add computation. Every CLI returns the workflow result, but the computed receipts can be also used to pull results directly from IPFS by CID.

If we post the workflow to the gateway again, we see a different set of logs:

![add-replay-logs](assets/add-replay.png)

This time we don't need to do any work. Homestar cached the receipts from our last run, and reports that it is replaying the workflow and its receipts.

Notice also that our WASI log does not show up. WASI logs only happen on execution, not replay. We'll see in a moment how we can force re-execution to always see WASI logs.

Let's try a workflow that uses all four arithmetic operations from our Rust, JavaScript, and Python sourced components:

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
    },
    {
      "run": {
        "name": "subtract",
        "input": {
          "args": ["{{needs.add.output}}", 4.4],
          "func": "subtract"
        }
      }
    },
    {
      "run": {
        "name": "multiply",
        "input": {
          "args": ["{{needs.subtract.output}}", 2.3],
          "func": "multiply"
        }
      }
    },
    {
      "run": {
        "name": "divide",
        "input": {
          "args": ["{{needs.multiply.output}}", 1.5],
          "func": "divide"
        }
      }
    }
  ]
}
```

In this workflow, each task except the first receives an input from the previous task. For example, `subtract` awaits the output of `add` by using `"{{needs.add.output}}"` as a placeholder that will be filled in when `add` has completed.

Restart Every CLI, passing in all of our Wasm components:

```sh
every dev --fn rust/target/wasm32-wasi/release/math.wasm --fn javascript/output/subtract-j54di3rspj2eewjro4.wasm --fn python/output/multiply.wasm --debug
```

The hash of your subtract Wasm component may be different. Check `javascript/output` for the appropriate file name.

We use the `--debug` flag this time to force re-execution of the tasks in our workflow. The `--debug` flag lets us see our WASI logs on every run while we are developing our functions, but should not be used in production because it eliminates the benefits of caching.

Post this workflow:

```sh
curl localhost:3000/run --json @workflows/all.json
```

The response reports a result of `5.98` which looks close enough for computer math!

Our WASI logging reports each operation:

![all-logs](assets/all.png)

We can see WASI logs from each of our components, labeled by category as `guest:rust:add`, `guest:javascript:subtract`, `guest:python:multiply`, and `guest:rust:divide`.

Lastly, a workflow that attempts division by zero to check our error reporting.

```json
{
  "tasks": [
    {
      "run": {
        "name": "divide",
        "input": {
          "args": [3.1, 0.0],
          "func": "divide"
        }
      }
    }
  ]
}
```

On running this workflow, we see two errors:

![division-by-zero](assets/division-by-zero.png)

The first error is our WASI log reporting a "Division by zero error". The second error is an execution error from the Wasm runtime. It's a bit inscutable, but we can see "not able to run fn divide" which tells us which function failed.

We've used default Homestar settings while running workflows, but these settings can be overriden with the `--config` option.

```sh
every dev --config settings.toml
```

See the [Homestar configuration docs][homestar-config] for commonly used settings.

### Everywhere Computer Control Panel

You may have noticed Every CLI starts a Control Panel:

![control-panel](assets/control-panel.png)

We will share more about Control Panel in a future post.

### Conclusion

#### Acknowledgements

We'd like to offer our sincere thanks to

[^1]: Other supported languages include C/C++, Java (TeaVM Java), Go (TinyGo), and C#.

[beta-signup]: https://docs.google.com/forms/d/e/1FAIpQLSfREjmoTBOW2gyUSFypn3omifibvptH0K_IQwtFWiGORU5vAQ/viewform
[bytecode-alliance]: https://bytecodealliance.org/
[canonical-abi]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md
[cargo-component]: https://github.com/bytecodealliance/cargo-component
[cid]: https://docs.ipfs.tech/concepts/content-addressing/
[componentize-js]: https://github.com/bytecodealliance/ComponentizeJS
[componentize-py]: https://github.com/bytecodealliance/componentize-py
[content-addressing]: https://en.wikipedia.org/wiki/Content-addressable_storage
[core-wasm]: https://webassembly.github.io/spec/core/
[everycli]: https://docs.everywhere.computer/everycli/
[everywhere-comp]: https://everywhere.computer/
[fastly-compute]: https://www.fastly.com/products/compute
[fermyon-spin]: https://www.fermyon.com/spin
[homestar-client]: https://www.npmjs.com/package/@fission-codes/homestar
[homestar-config]: https://docs.everywhere.computer/homestar/configuration/
[homestar-runtime]: https://github.com/ipvm-wg/homestar/blob/main/README.md
[install-every-cli]: https://www.npmjs.com/package/@everywhere-computer/every-cli
[install-ipfs]: https://docs.ipfs.tech/install/command-line/#install-official-binary-distributions
[introducing-componentize-py-blog]: https://www.fermyon.com/blog/introducing-componentize-py
[introducing-componentize-py-video]: https://www.youtube.com/watch?v=PkAO17lmqsI
[ipfs]: https://ipfs.tech/
[ipvm]: https://fission.codes/ecosystem/ipvm/
[ir]: https://en.wikipedia.org/wiki/Intermediate_representation
[javascript-webassembly-post]: https://bytecodealliance.org/articles/making-javascript-run-fast-on-webassembly
[path-to-components]: https://youtu.be/phodPLY8zNE
[pop]: https://en.wikipedia.org/wiki/Point_of_presence
[simple-made-easy]: https://www.infoq.com/presentations/Simple-Made-Easy/
[wasm-component]: https://component-model.bytecodealliance.org/
[wasip2]: https://blog.sunfishcode.online/wasi-0-2/
[wasmtime]: https://docs.wasmtime.dev/
[wasmcloud]: https://wasmcloud.com/blog/wasmtime-a-standardized-runtime-for-wasmcloud
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[wit-bindgen]: https://github.com/bytecodealliance/wit-bindgen
[wit-guest]: https://github.com/bytecodealliance/wit-bindgen?tab=readme-ov-file#supported-guest-languages
[wit-to-ipld]: https://github.com/ipvm-wg/homestar/tree/main/homestar-wasm#interpreting-between-ipld-and-wit
[workflows]: https://aws.amazon.com/what-is/workflow/
[write-once-run]: https://youtu.be/dhoVlVu2XAw?si=x1YIQk-9Jkg_FphP
[writing-functions-repo]: https://github.com/everywhere-computer/writing-functions-blogpost-2024
