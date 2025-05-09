# WebAssembly-Template Example

This project is a webassembly template that can be run in a web browser using WebAssembly. The example can be executed in any Chromium-based browser such as Chrome, Edge, and Opera, which allow passing through an Iotzio device directly to the WebAssembly application.

## Running the Example

To compile and run the examples, follow these steps:

1. Navigate to the `examples/webassembly-template` directory:
    ```sh
    cd examples
    cd webassembly-template
    ```

2. Build the WebAssembly package using `wasm-pack`:
    ```sh
    wasm-pack build --release --no-typescript --out-dir /pkg --target web
    ```

3. Serve the generated pkg folder and included index.html file using a **HTTPS enabled** web server of your choice and open the example in a Chromium-based browser.

## Requirements

- Rust and Cargo installed
- `wasm-pack` installed (`cargo install wasm-pack`)
- A Chromium-based browser (Chrome, Edge, Opera)
