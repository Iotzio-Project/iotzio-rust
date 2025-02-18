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

## Notes

- Ensure that the browser has WebHID enabled to allow communication with Iotzio devices.
- Browsers running on Linux with Snap packages may not work if they lack the necessary permissions.
- On Linux, it is necessary to grant read and write permissions for the Iotzio device:

    ```sh
    sudo usermod -a -G dialout YOUR_USERNAME
    ```

    ```sh
    echo 'KERNEL=="hidraw*", SUBSYSTEM=="hidraw", ATTRS{idVendor}=="2e8a", ATTRS{idProduct}=="000f", GROUP="dialout", MODE="0660"' | sudo tee /etc/udev/rules.d/99-iotzio.rules
    ```

    ```sh
    echo 'SUBSYSTEM=="usb", ATTRS{idVendor}=="2e8a", ATTRS{idProduct}=="000f", GROUP="dialout", MODE="0660"' | sudo tee -a /etc/udev/rules.d/99-iotzio.rules
    ```

    ```sh
    sudo udevadm control --reload-rules
    ```

    ```sh
    sudo udevadm trigger
    ```