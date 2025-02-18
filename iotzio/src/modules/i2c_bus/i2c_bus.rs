use crate::modules::i2c_bus::{service, I2cBusModuleError, I2cConfig};
use crate::peripherals::i2c::I2cIdentifier;
use crate::socket::Socket;
use async_std::sync::Mutex;
use async_std::task::block_on;
use std::sync::Arc;

#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Object))]
#[derive(Debug)]
pub struct I2cBus {
    pub(crate) socket: Arc<Socket>,
    pub(crate) mutex: Mutex<()>,
    pub(crate) identifier: I2cIdentifier,
}

impl I2cBus {
    #[inline]
    pub(crate) async fn new(socket: &Arc<Socket>, config: I2cConfig) -> Result<I2cBus, I2cBusModuleError> {
        service::new(socket, config).await
    }
}

#[cfg(all(not(target_family = "wasm"), not(feature = "_uniffi-blocking")))]
impl I2cBus {
    /// Read from address into buffer.
    #[inline]
    pub fn read(&self, address: u16, buffer: &mut [u8]) -> Result<(), I2cBusModuleError> {
        block_on(service::read(
            &self.socket,
            &self.mutex,
            self.identifier,
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
            self.identifier,
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
            self.identifier,
            address,
            bytes,
            buffer,
        ))
    }
}

#[cfg(all(not(target_family = "wasm"), feature = "_uniffi-blocking"))]
#[uniffi::export]
impl I2cBus {
    /// Read from address into buffer. Returns buffer.
    #[inline]
    pub fn read(&self, address: u16, mut buffer: Vec<u8>) -> Result<Vec<u8>, I2cBusModuleError> {
        block_on(service::read(
            &self.socket,
            &self.mutex,
            self.identifier,
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
            self.identifier,
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
            self.identifier,
            address,
            bytes.as_slice(),
            buffer.as_mut_slice(),
        ))
        .map(|_| buffer)
    }
}

#[cfg(not(feature = "_uniffi-async"))]
impl I2cBus {
    /// Read from address into buffer.
    #[inline]
    pub async fn read_async(&self, address: u16, buffer: &mut [u8]) -> Result<(), I2cBusModuleError> {
        service::read(&self.socket, &self.mutex, self.identifier, address, buffer).await
    }

    /// Write to address from bytes.
    #[inline]
    pub async fn write_async(&self, address: u16, bytes: &[u8]) -> Result<(), I2cBusModuleError> {
        service::write(&self.socket, &self.mutex, self.identifier, address, bytes).await
    }

    /// Write to address from bytes, read from address into buffer.
    #[inline]
    pub async fn write_read_async(&self, address: u16, write: &[u8], read: &mut [u8]) -> Result<(), I2cBusModuleError> {
        service::write_read(&self.socket, &self.mutex, self.identifier, address, write, read).await
    }
}

#[cfg(feature = "_uniffi-async")]
#[uniffi::export]
impl I2cBus {
    /// Read from address into buffer.
    #[inline]
    pub async fn read_async(&self, address: u16, mut buffer: Vec<u8>) -> Result<Vec<u8>, I2cBusModuleError> {
        service::read(
            &self.socket,
            &self.mutex,
            self.identifier,
            address,
            buffer.as_mut_slice(),
        )
        .await
        .map(|_| buffer)
    }

    /// Write to address from bytes.
    #[inline]
    pub async fn write_async(&self, address: u16, bytes: Vec<u8>) -> Result<(), I2cBusModuleError> {
        service::write(&self.socket, &self.mutex, self.identifier, address, bytes.as_slice()).await
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
            self.identifier,
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
        let bus = self.identifier;

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
            self.identifier,
            address as u16,
            read,
        ))
    }

    #[inline]
    fn write(&mut self, address: embedded_hal::i2c::SevenBitAddress, write: &[u8]) -> Result<(), Self::Error> {
        block_on(service::write(
            &self.socket,
            &self.mutex,
            self.identifier,
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
            self.identifier,
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
            self.identifier,
            address as u16,
            operations,
        ))
    }
}

#[cfg(feature = "embedded-hal")]
impl embedded_hal_async::i2c::I2c for I2cBus {
    #[inline]
    async fn read(&mut self, address: embedded_hal::i2c::SevenBitAddress, read: &mut [u8]) -> Result<(), Self::Error> {
        service::read(&self.socket, &self.mutex, self.identifier, address as u16, read).await
    }

    #[inline]
    async fn write(&mut self, address: embedded_hal::i2c::SevenBitAddress, write: &[u8]) -> Result<(), Self::Error> {
        service::write(&self.socket, &self.mutex, self.identifier, address as u16, write).await
    }

    #[inline]
    async fn write_read(
        &mut self,
        address: embedded_hal::i2c::SevenBitAddress,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        service::write_read(&self.socket, &self.mutex, self.identifier, address as u16, write, read).await
    }

    #[inline]
    async fn transaction(
        &mut self,
        address: embedded_hal::i2c::SevenBitAddress,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        service::transaction(&self.socket, &self.mutex, self.identifier, address as u16, operations).await
    }
}
