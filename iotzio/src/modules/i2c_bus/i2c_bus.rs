use crate::modules::i2c_bus::{service, I2cBusModuleError, I2cConfig};
use crate::peripherals::i2c::I2cBusNumber;
use crate::socket::Socket;
use async_std::sync::Mutex;
use async_std::task::block_on;
use std::sync::Arc;

/// Represents an I2C bus on the Iotzio device.
/// With this module you can communicate directly to bus participants.
/// Use this also if you want to create specialized I2C bus dependent modules.
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Object))]
#[derive(Debug)]
pub struct I2cBus {
    pub(crate) socket: Arc<Socket>,
    pub(crate) mutex: Mutex<()>,
    pub(crate) bus_number: I2cBusNumber,
}

#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), uniffi::export)]
impl I2cBus {
    /// The I2C bus number this instance is using.
    #[inline]
    pub fn bus_number(&self) -> I2cBusNumber {
        self.bus_number.clone()
    }
}

impl I2cBus {
    #[inline]
    pub(crate) async fn new(socket: &Arc<Socket>, config: I2cConfig) -> Result<I2cBus, I2cBusModuleError> {
        service::new(socket, config).await
    }
}

#[cfg(all(not(target_family = "wasm"), not(feature = "_ffi-blocking")))]
impl I2cBus {
    /// Read from address into buffer.
    #[inline]
    pub fn read(&self, address: u16, buffer: &mut [u8]) -> Result<(), I2cBusModuleError> {
        block_on(service::read(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address,
            buffer,
        ))
    }

    /// Write to address from bytes.
    #[inline]
    pub fn write(&self, address: u16, bytes: &[u8]) -> Result<(), I2cBusModuleError> {
        block_on(service::write(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address,
            bytes,
        ))
    }

    /// Write to address from bytes, read from address into buffer.
    #[inline]
    pub fn write_read(&self, address: u16, bytes: &[u8], buffer: &mut [u8]) -> Result<(), I2cBusModuleError> {
        block_on(service::write_read(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address,
            bytes,
            buffer,
        ))
    }
}

#[cfg(all(not(target_family = "wasm"), feature = "_ffi-blocking"))]
#[uniffi::export]
impl I2cBus {
    /// Read from address into buffer. Returns buffer.
    #[inline]
    pub fn read(&self, address: u16, mut buffer: Vec<u8>) -> Result<Vec<u8>, I2cBusModuleError> {
        block_on(service::read(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address,
            buffer.as_mut_slice(),
        ))
        .map(|_| buffer)
    }

    /// Write to address from bytes.
    #[inline]
    pub fn write(&self, address: u16, bytes: Vec<u8>) -> Result<(), I2cBusModuleError> {
        block_on(service::write(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address,
            bytes.as_slice(),
        ))
    }

    /// Write to address from bytes, read from address into buffer. Returns buffer.
    #[inline]
    pub fn write_read(&self, address: u16, bytes: Vec<u8>, mut buffer: Vec<u8>) -> Result<Vec<u8>, I2cBusModuleError> {
        block_on(service::write_read(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address,
            bytes.as_slice(),
            buffer.as_mut_slice(),
        ))
        .map(|_| buffer)
    }
}

#[cfg(not(feature = "_ffi-async"))]
impl I2cBus {
    /// Read from address into buffer.
    #[inline]
    pub async fn read_async(&self, address: u16, buffer: &mut [u8]) -> Result<(), I2cBusModuleError> {
        service::read(&self.socket, &self.mutex, self.bus_number, address, buffer).await
    }

    /// Write to address from bytes.
    #[inline]
    pub async fn write_async(&self, address: u16, bytes: &[u8]) -> Result<(), I2cBusModuleError> {
        service::write(&self.socket, &self.mutex, self.bus_number, address, bytes).await
    }

    /// Write to address from bytes, read from address into buffer.
    #[inline]
    pub async fn write_read_async(&self, address: u16, write: &[u8], read: &mut [u8]) -> Result<(), I2cBusModuleError> {
        service::write_read(&self.socket, &self.mutex, self.bus_number, address, write, read).await
    }
}

#[cfg(feature = "_ffi-async")]
#[uniffi::export]
impl I2cBus {
    /// Read from address into buffer.
    #[inline]
    pub async fn read_async(&self, address: u16, mut buffer: Vec<u8>) -> Result<Vec<u8>, I2cBusModuleError> {
        service::read(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address,
            buffer.as_mut_slice(),
        )
        .await
        .map(|_| buffer)
    }

    /// Write to address from bytes.
    #[inline]
    pub async fn write_async(&self, address: u16, bytes: Vec<u8>) -> Result<(), I2cBusModuleError> {
        service::write(&self.socket, &self.mutex, self.bus_number, address, bytes.as_slice()).await
    }

    /// Write to address from bytes, read from address into buffer. Returns buffer.
    #[inline]
    pub async fn write_read_async(
        &self,
        address: u16,
        bytes: Vec<u8>,
        mut buffer: Vec<u8>,
    ) -> Result<Vec<u8>, I2cBusModuleError> {
        service::write_read(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address,
            bytes.as_slice(),
            buffer.as_mut_slice(),
        )
        .await
        .map(|_| buffer)
    }
}

impl Drop for I2cBus {
    #[inline]
    fn drop(&mut self) {
        let socket = self.socket.clone();
        let bus = self.bus_number;

        block_on(async move { _ = service::drop(&socket, bus).await })
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal::i2c::ErrorType for I2cBus {
    type Error = I2cBusModuleError;
}

#[cfg(all(feature = "embedded-hal", not(target_family = "wasm")))]
impl embedded_hal::i2c::I2c for I2cBus {
    #[inline]
    fn read(&mut self, address: embedded_hal::i2c::SevenBitAddress, read: &mut [u8]) -> Result<(), Self::Error> {
        block_on(service::read(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address as u16,
            read,
        ))
    }

    #[inline]
    fn write(&mut self, address: embedded_hal::i2c::SevenBitAddress, write: &[u8]) -> Result<(), Self::Error> {
        block_on(service::write(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address as u16,
            write,
        ))
    }

    #[inline]
    fn write_read(
        &mut self,
        address: embedded_hal::i2c::SevenBitAddress,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        block_on(service::write_read(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address as u16,
            write,
            read,
        ))
    }

    #[inline]
    fn transaction(
        &mut self,
        address: embedded_hal::i2c::SevenBitAddress,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        block_on(service::transaction(
            &self.socket,
            &self.mutex,
            self.bus_number,
            address as u16,
            operations,
        ))
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal_async::i2c::I2c for I2cBus {
    #[inline]
    async fn read(&mut self, address: embedded_hal::i2c::SevenBitAddress, read: &mut [u8]) -> Result<(), Self::Error> {
        service::read(&self.socket, &self.mutex, self.bus_number, address as u16, read).await
    }

    #[inline]
    async fn write(&mut self, address: embedded_hal::i2c::SevenBitAddress, write: &[u8]) -> Result<(), Self::Error> {
        service::write(&self.socket, &self.mutex, self.bus_number, address as u16, write).await
    }

    #[inline]
    async fn write_read(
        &mut self,
        address: embedded_hal::i2c::SevenBitAddress,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        service::write_read(&self.socket, &self.mutex, self.bus_number, address as u16, write, read).await
    }

    #[inline]
    async fn transaction(
        &mut self,
        address: embedded_hal::i2c::SevenBitAddress,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        service::transaction(&self.socket, &self.mutex, self.bus_number, address as u16, operations).await
    }
}
