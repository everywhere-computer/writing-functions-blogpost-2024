# Python

For Python, we implement a multiply function. The multiply function multiplies two signed integers and returns a signed integer result. It also logs the multiplication operation.

## Setup

Install the latest version of [Python][install-python]. Python 3.10 is the minimum required version for [`componentize-py`][componentize-py].

Activate a virtual environment using the method of your choosing and install the dependencies:

```sh
pip install -r requirements.txt
```

## Build

Build using `componentize-py`:

```sh
componentize-py -d wit -w maths componentize app -o output/multiply.wasm
```

The `-d` option is the directory with our WIT definitions and `-w` is the [`maths` world][maths-wit] we are building.

The `componentize` operation targets `app.py` and outputs a Wasm component to the `output` directory.

[componentize-py]: https://github.com/bytecodealliance/componentize-py
[install-python]: https://www.python.org/downloads/
[maths-wit]: ./wit/world.wit
