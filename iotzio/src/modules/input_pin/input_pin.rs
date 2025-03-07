use crate::modules::input_pin::{service, InputPinModuleError};
use crate::peripherals::gpio::{GpioPin, Level, Pull};
use crate::socket::Socket;
use async_std::task::block_on;
use std::sync::Arc;
use std::time::Duration;

#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Object))]
#[derive(Debug)]
pub struct InputPin {
    pub(crate) socket: Arc<Socket>,
    pub(crate) pin: GpioPin,
    pub(crate) pull_setting: Pull,
    pub(crate) hysteresis: bool,
}

impl InputPin {
    #[inline]
    pub(crate) async fn new(
        socket: &Arc<Socket>,
        pin: GpioPin,
        pull_setting: Pull,
        hysteresis: bool,
    ) -> Result<InputPin, InputPinModuleError> {
        service::new(socket, pin, pull_setting, hysteresis).await
    }
}

#[cfg_attr(feature = "_ffi-blocking", uniffi::export)]
impl InputPin {
    /// Returns used pin.
    #[inline]
    pub fn get_pin(&self) -> GpioPin {
        self.pin
    }

    /// Returns current pull setting.
    #[inline]
    pub fn get_pull_setting(&self) -> Pull {
        self.pull_setting
    }

    /// Returns whether hysteresis is enabled.
    #[inline]
    pub fn is_hysteresis_enabled(&self) -> bool {
        self.hysteresis
    }
}

#[cfg_attr(feature = "_ffi-async", uniffi::export)]
impl InputPin {
    /// Returns current pin level.
    #[inline]
    pub async fn get_level_async(&self) -> Result<Level, InputPinModuleError> {
        service::get_level(&self.socket, self.pin).await
    }

    /// Get whether the pin input level is high.
    #[inline]
    pub async fn is_high_async(&self) -> Result<bool, InputPinModuleError> {
        service::is_high(&self.socket, self.pin).await
    }

    /// Get whether the pin input level is low.
    #[inline]
    pub async fn is_low_async(&self) -> Result<bool, InputPinModuleError> {
        service::is_low(&self.socket, self.pin).await
    }

    /// Wait until the pin is high. If it is already high, return immediately.
    #[inline]
    pub async fn wait_for_high_async(&self) -> Result<(), InputPinModuleError> {
        service::wait_for_high(&self.socket, self.pin).await
    }

    /// Wait until the pin is low. If it is already low, return immediately.
    #[inline]
    pub async fn wait_for_low_async(&self) -> Result<(), InputPinModuleError> {
        service::wait_for_low(&self.socket, self.pin).await
    }

    /// Wait for the pin to undergo a transition from low to high.
    #[inline]
    pub async fn wait_for_rising_edge_async(&self) -> Result<(), InputPinModuleError> {
        service::wait_for_rising_edge(&self.socket, self.pin).await
    }

    /// Wait for the pin to undergo a transition from high to low.
    #[inline]
    pub async fn wait_for_falling_edge_async(&self) -> Result<(), InputPinModuleError> {
        service::wait_for_falling_edge(&self.socket, self.pin).await
    }

    /// Wait for the pin to undergo any transition, i.e. low to high OR high to low.
    #[inline]
    pub async fn wait_for_any_edge_async(&self) -> Result<(), InputPinModuleError> {
        service::wait_for_any_edge(&self.socket, self.pin).await
    }

    /// Wait for the pin to undergo a pulse transition from low to high to low again. Returns pulse width when succeeded.
    #[inline]
    pub async fn wait_for_high_pulse_async(&self) -> Result<Duration, InputPinModuleError> {
        service::wait_for_high_pulse(&self.socket, self.pin).await
    }

    /// Wait for the pin to undergo a pulse transition from high to low to high again. Returns pulse width when succeeded.
    #[inline]
    pub async fn wait_for_low_pulse_async(&self) -> Result<Duration, InputPinModuleError> {
        service::wait_for_low_pulse(&self.socket, self.pin).await
    }

    /// Wait for the pin to undergo a pulse transition, i.e. from low to high to low again OR from high to low to high again. Returns pulse width when succeeded.
    #[inline]
    pub async fn wait_for_any_pulse_async(&self) -> Result<Duration, InputPinModuleError> {
        service::wait_for_any_pulse(&self.socket, self.pin).await
    }
}

#[cfg(not(target_family = "wasm"))]
#[cfg_attr(feature = "_ffi-blocking", uniffi::export)]
impl InputPin {
    /// Returns current pin level.
    #[inline]
    pub fn get_level(&self) -> Result<Level, InputPinModuleError> {
        block_on(service::get_level(&self.socket, self.pin))
    }

    /// Get whether the pin input level is high.
    #[inline]
    pub fn is_high(&self) -> Result<bool, InputPinModuleError> {
        block_on(service::is_high(&self.socket, self.pin))
    }

    /// Get whether the pin input level is low.
    #[inline]
    pub fn is_low(&self) -> Result<bool, InputPinModuleError> {
        block_on(service::is_low(&self.socket, self.pin))
    }

    /// Wait until the pin is high. If it is already high, return immediately.
    #[inline]
    pub fn wait_for_high(&self) -> Result<(), InputPinModuleError> {
        block_on(service::wait_for_high(&self.socket, self.pin))
    }

    /// Wait until the pin is low. If it is already low, return immediately.
    #[inline]
    pub fn wait_for_low(&self) -> Result<(), InputPinModuleError> {
        block_on(service::wait_for_low(&self.socket, self.pin))
    }

    /// Wait for the pin to undergo a transition from low to high.
    #[inline]
    pub fn wait_for_rising_edge(&self) -> Result<(), InputPinModuleError> {
        block_on(service::wait_for_rising_edge(&self.socket, self.pin))
    }

    /// Wait for the pin to undergo a transition from high to low.
    #[inline]
    pub fn wait_for_falling_edge(&self) -> Result<(), InputPinModuleError> {
        block_on(service::wait_for_falling_edge(&self.socket, self.pin))
    }

    /// Wait for the pin to undergo any transition, i.e low to high OR high to low.
    #[inline]
    pub fn wait_for_any_edge(&self) -> Result<(), InputPinModuleError> {
        block_on(service::wait_for_any_edge(&self.socket, self.pin))
    }

    /// Wait for the pin to undergo a pulse transition from low to high to low again. Returns pulse width when succeeded.
    #[inline]
    pub fn wait_for_high_pulse(&self) -> Result<Duration, InputPinModuleError> {
        block_on(service::wait_for_high_pulse(&self.socket, self.pin))
    }

    /// Wait for the pin to undergo a pulse transition from high to low to high again. Returns pulse width when succeeded.
    #[inline]
    pub fn wait_for_low_pulse(&self) -> Result<Duration, InputPinModuleError> {
        block_on(service::wait_for_low_pulse(&self.socket, self.pin))
    }

    /// Wait for the pin to undergo a pulse transition, i.e. from low to high to low again OR from high to low to high again. Returns pulse width when succeeded.
    #[inline]
    pub fn wait_for_any_pulse(&self) -> Result<Duration, InputPinModuleError> {
        block_on(service::wait_for_any_pulse(&self.socket, self.pin))
    }
}

impl Drop for InputPin {
    #[inline]
    fn drop(&mut self) {
        let socket = self.socket.clone();
        let pin = self.pin;

        block_on(async move { _ = service::drop(&socket, pin).await })
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::digital::ErrorType for InputPin {
    type Error = InputPinModuleError;
}

#[cfg(all(feature = "embedded-hal", not(target_family = "wasm")))]
impl embedded_hal::digital::InputPin for InputPin {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        block_on(service::is_high(&self.socket, self.pin))
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        block_on(service::is_low(&self.socket, self.pin))
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal_async::digital::Wait for InputPin {
    #[inline]
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        service::wait_for_high(&self.socket, self.pin).await
    }

    #[inline]
    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        service::wait_for_low(&self.socket, self.pin).await
    }

    #[inline]
    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        service::wait_for_rising_edge(&self.socket, self.pin).await
    }

    #[inline]
    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        service::wait_for_falling_edge(&self.socket, self.pin).await
    }

    #[inline]
    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        service::wait_for_any_edge(&self.socket, self.pin).await
    }
}
