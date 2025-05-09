# Iotzio

The Iotzio API allows interaction with Iotzio devices. An Iotzio device is a USB connected microchip that enables the host computer to directly control peripherals such as GPIOs, utilize PWM, use I2C, SPI, Onewire and other bus protocols and devices that are not typically available to an application developer on a standard computer. This API is also available to many other programming languages. No extra drivers required!

## Features

- Control GPIOs, utilize PWM, use I2C, SPI, Onewire and other bus protocols
- Direct interaction for various peripherals
- Available for multiple programming languages
- The Iotzio API in its core is using pure idiomatic Rust - blazingly fast and memory safe

## Compatibility

The Iotzio board is compatible with the following platforms:
- Windows (10, 11)
- Linux (Kernel >= 3.0)
- macOS (>= Catalina 10.15)
- Android (Version >= 8.0 / API Level >= 26)
- Browser ([WebHID support required](https://developer.mozilla.org/en-US/docs/Web/API/WebHID_API#browser_compatibility))

## Installation

Iotzio is available on [crates.io](https://crates.io/crates/iotzio). Just add `iotzio` as dependency to your `Cargo.toml`. Integration of the embedded-hal(-async) traits is available using crate feature `embedded-hal`. Rust Edition `2021` and `2024` is supported.

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

- On some USB type C ports, the Iotzio device may not be recognized, as USB 1.1 devices are not officially supported by the USB-C standard. While many manufacturers still accommodate them, Apple® for examples does not. A workaround is to use a USB hub in between or just a Type A port.


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
    sudo reboot
    ```

- On Linux, you might encounter difficulties using the Iotzio device in browsers distributed as Snap packages, as they often lack the necessary permissions.
