use crate::communication::{BoardInfo, Version};
use crate::modules;
use crate::peripherals::gpio::{Drive, GpioPin, Level, Pull, SlewRate};
use crate::socket::Socket;
use std::ops::Deref;
use std::sync::Arc;

/// The representation of an opened Iotzio device.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Object))]
#[derive(Debug)]
pub struct Iotzio {
    pub(crate) socket: Arc<Socket>,
    pub(crate) board_info: BoardInfo,
}

#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), uniffi::export)]
impl Iotzio {
    /// The persistent and unique serial number of the Iotzio device.
    #[inline]
    pub fn serial_number(&self) -> String {
        self.board_info.serial_number.clone()
    }

    /// The semantic version of the Iotzio device.
    #[inline]
    pub fn version(&self) -> Version {
        self.board_info.version
    }

    /// The protocol version of the iotzio device. This may increase during a major version update. Library and device must match protocol version.
    #[inline]
    pub fn protocol_version(&self) -> u16 {
        self.board_info.protocol_version
    }

    /// The runtime identifier of the Iotzio device. As long as the device remains connected, the runtime identifier stays consistent. However, it may potentially change after the physical USB connection is reestablished.
    #[inline]
    pub fn runtime_identifier(&self) -> u64 {
        self.socket.runtime_identifier.deref().clone()
    }
}

#[cfg(not(target_family = "wasm"))]
#[cfg_attr(feature = "_ffi-blocking", uniffi::export)]
impl Iotzio {
    /// Set up a new input pin with the given pull setting. Hysteresis enabled ensures reliable signal interpretation, even with noisy or slowly changing input signals.
    /// During the existence of the returned module instance, the pin cannot be used for other modules.
    #[inline]
    pub fn setup_input_pin(
        &self,
        pin: GpioPin,
        pull_setting: Pull,
        hysteresis: bool,
    ) -> Result<modules::input_pin::InputPin, modules::input_pin::InputPinModuleError> {
        async_std::task::block_on(modules::input_pin::InputPin::new(
            &self.socket,
            pin,
            pull_setting,
            hysteresis,
        ))
    }

    /// Set up a new output pin with given initial level, drive strength and slew rate.
    /// During the existence of the returned module instance, the pin cannot be used for other modules.
    #[inline]
    pub fn setup_output_pin(
        &self,
        pin: GpioPin,
        initial_level: Level,
        drive_strength: Drive,
        slew_rate: SlewRate,
    ) -> Result<modules::output_pin::OutputPin, modules::output_pin::OutputPinModuleError> {
        async_std::task::block_on(modules::output_pin::OutputPin::new(
            &self.socket,
            pin,
            initial_level,
            drive_strength,
            slew_rate,
        ))
    }

    /// Set up a new I2C bus using the given I2C configuration.
    /// With this module instance you can communicate directly to bus participants.
    /// Use this method also if you want to create specialized I2C bus dependent modules.
    #[inline]
    pub fn setup_i2c_bus(
        &self,
        config: modules::i2c_bus::I2cConfig,
    ) -> Result<modules::i2c_bus::I2cBus, modules::i2c_bus::I2cBusModuleError> {
        async_std::task::block_on(modules::i2c_bus::I2cBus::new(&self.socket, config))
    }
}

#[cfg_attr(feature = "_ffi-async", uniffi::export)]
impl Iotzio {
    /// Set up a new input pin with the given pull setting. Hysteresis enabled ensures reliable signal interpretation, even with noisy or slowly changing input signals.
    /// During the existence of the returned module instance, the pin cannot be used for other modules.
    #[inline]
    pub async fn setup_input_pin_async(
        &self,
        pin: GpioPin,
        pull_setting: Pull,
        hysteresis: bool,
    ) -> Result<modules::input_pin::InputPin, modules::input_pin::InputPinModuleError> {
        modules::input_pin::InputPin::new(&self.socket, pin, pull_setting, hysteresis).await
    }

    /// Set up a new output pin with given initial level, drive strength and slew rate.
    /// During the existence of the returned module instance, the pin cannot be used for other modules.
    #[inline]
    pub async fn setup_output_pin_async(
        &self,
        pin: GpioPin,
        initial_level: Level,
        drive_strength: Drive,
        slew_rate: SlewRate,
    ) -> Result<modules::output_pin::OutputPin, modules::output_pin::OutputPinModuleError> {
        modules::output_pin::OutputPin::new(&self.socket, pin, initial_level, drive_strength, slew_rate).await
    }

    /// Set up a new I2C bus using the given I2C configuration.
    /// With this module instance you can communicate directly to bus participants.
    /// Use this method also if you want to create specialized I2C bus dependent modules.
    #[inline]
    pub async fn setup_i2c_bus_async(
        &self,
        config: modules::i2c_bus::I2cConfig,
    ) -> Result<modules::i2c_bus::I2cBus, modules::i2c_bus::I2cBusModuleError> {
        modules::i2c_bus::I2cBus::new(&self.socket, config).await
    }
}
