use crate::communication::{BoardInfo, Version};
use crate::modules;
use crate::peripherals::gpio::{Drive, GpioPin, Level, Pull, SlewRate};
use crate::socket::Socket;
use std::ops::Deref;
use std::sync::Arc;

#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Object))]
#[derive(Debug)]
pub struct Iotzio {
    pub(crate) socket: Arc<Socket>,
    pub(crate) board_info: BoardInfo,
}

#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), uniffi::export)]
impl Iotzio {
    #[inline]
    pub fn serial_number(&self) -> String {
        self.board_info.serial_number.clone()
    }

    #[inline]
    pub fn version(&self) -> Version {
        self.board_info.version
    }

    #[inline]
    pub fn protocol_version(&self) -> u16 {
        self.board_info.protocol_version
    }

    #[inline]
    pub fn runtime_identifier(&self) -> u64 {
        self.socket.runtime_identifier.deref().clone()
    }
}

#[cfg(not(target_family = "wasm"))]
#[cfg_attr(feature = "_uniffi-blocking", uniffi::export)]
impl Iotzio {
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

    #[inline]
    pub fn setup_i2c_bus(
        &self,
        config: modules::i2c_bus::I2cConfig,
    ) -> Result<modules::i2c_bus::I2cBus, modules::i2c_bus::I2cBusModuleError> {
        async_std::task::block_on(modules::i2c_bus::I2cBus::new(&self.socket, config))
    }
}

#[cfg_attr(feature = "_uniffi-async", uniffi::export)]
impl Iotzio {
    #[inline]
    pub async fn setup_input_pin_async(
        &self,
        pin: GpioPin,
        pull_setting: Pull,
        hysteresis: bool,
    ) -> Result<modules::input_pin::InputPin, modules::input_pin::InputPinModuleError> {
        modules::input_pin::InputPin::new(&self.socket, pin, pull_setting, hysteresis).await
    }

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

    #[inline]
    pub async fn setup_i2c_bus_async(
        &self,
        config: modules::i2c_bus::I2cConfig,
    ) -> Result<modules::i2c_bus::I2cBus, modules::i2c_bus::I2cBusModuleError> {
        modules::i2c_bus::I2cBus::new(&self.socket, config).await
    }
}
