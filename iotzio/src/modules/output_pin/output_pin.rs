use crate::modules::output_pin::{service, OutputPinModuleError};
use crate::peripherals::gpio::{Drive, GpioPin, Level, SlewRate};
use crate::socket::Socket;
use async_std::task::block_on;
use std::sync::{Arc, Mutex};

#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Object))]
#[derive(Debug)]
pub struct OutputPin {
    pub(crate) socket: Arc<Socket>,
    pub(crate) pin: GpioPin,
    pub(crate) level: Mutex<Level>,
    pub(crate) drive_strength: Drive,
    pub(crate) slew_rate: SlewRate,
}

impl OutputPin {
    #[inline]
    pub(crate) async fn new(
        socket: &Arc<Socket>,
        pin: GpioPin,
        initial_level: Level,
        drive_strength: Drive,
        slew_rate: SlewRate,
    ) -> Result<OutputPin, OutputPinModuleError> {
        service::new(socket, pin, initial_level, drive_strength, slew_rate).await
    }
}

#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), uniffi::export)]
impl OutputPin {
    /// Returns used pin.
    #[inline]
    pub fn get_pin(&self) -> GpioPin {
        self.pin
    }

    /// Returns current pin level.
    #[inline]
    pub fn get_level(&self) -> Level {
        self.level.lock().unwrap().clone()
    }

    /// Returns the pin's drive strength.
    #[inline]
    pub fn get_drive_strength(&self) -> Drive {
        self.drive_strength
    }

    /// Returns the pin's slew rate.
    #[inline]
    pub fn get_slew_rate(&self) -> SlewRate {
        self.slew_rate
    }
}

#[cfg_attr(feature = "_uniffi-async", uniffi::export)]
impl OutputPin {
    /// Sets current pin level.
    #[inline]
    pub async fn set_level_async(&self, level: Level) -> Result<(), OutputPinModuleError> {
        service::set_level(&self.socket, &self.level, self.pin, level).await
    }
}

#[cfg(not(target_family = "wasm"))]
#[cfg_attr(feature = "_uniffi-blocking", uniffi::export)]
impl OutputPin {
    /// Sets current pin level.
    #[inline]
    pub fn set_level(&self, level: Level) -> Result<(), OutputPinModuleError> {
        block_on(service::set_level(&self.socket, &self.level, self.pin, level))
    }
}

impl Drop for OutputPin {
    #[inline]
    fn drop(&mut self) {
        let socket = self.socket.clone();
        let pin = self.pin;

        block_on(async move { _ = service::drop(&socket, pin).await })
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::ErrorType for OutputPin {
    type Error = OutputPinModuleError;
}

#[cfg(all(feature = "embedded-hal", not(target_family = "wasm")))]
impl embedded_hal::digital::OutputPin for OutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        block_on(service::set_level(&self.socket, &self.level, self.pin, Level::Low))
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        block_on(service::set_level(&self.socket, &self.level, self.pin, Level::High))
    }
}

#[cfg(all(feature = "embedded-hal", not(target_family = "wasm")))]
impl embedded_hal::digital::StatefulOutputPin for OutputPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.level.lock().unwrap().eq(&Level::High))
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.level.lock().unwrap().eq(&Level::Low))
    }
}
