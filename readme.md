# Iotzio

The `iotzio` crate allows interaction with Iotzio devices. An Iotzio device is a microcontroller that enables the host computer to directly control peripherals such as GPIO, I2C, SPI, Onewire, and many other bus protocols and devices that are not typically available to an application developer on a standard computer. This Rust library is the reference implementation. There are bindings available for many other programming languages. No extra drivers required!

## Features

- Control GPIOs, utilize PWM, use I2C, SPI, Onewire and other bus protocols
- Direct interaction for various peripherals
- Reference implementation in idiomatic Rust - blazingly fast and memory safe
- Bindings available for multiple programming languages

## Compatibility

The Iotzio board is compatible with the following platforms:
- Windows
- Linux
- macOS
- Android
- WebAssembly

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
iotzio = "0.1.0"
```

This crate provides integration of the embedded-hal(-async) traits using feature `embedded-hal`.

## Usage
Here is a simple example of how to use the iotzio crate:
```
pub fn main() {
    let iotzio_infos = iotzio::IotzioManager::new().list_connected_boards().unwarp();

    for iotzio_info in iotzio_infos {
        let iotzio = iotzio_info.open().unwarp();

        println!("Found Iotzio {0} with serial number {1}!", iotzio.version(), iotzio.serial_number());
    }
}
```

Further examples are located in the [examples folder](https://github.com/Iotzio-Project/iotzio-rust/tree/main/examples).

## Notes

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