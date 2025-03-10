use crate::communication::{Command, FatalError, ProtocolError, Response};
use crate::modules::i2c_bus::{I2cBus, I2cBusModuleError, I2cConfig};
use crate::peripherals::i2c::I2cBusNumber;
use crate::peripherals::{BusBuffer, BUS_BUFFER_SIZE};
use crate::socket::Socket;
use async_std::sync::Mutex;
use async_std::task::block_on;
use std::sync::Arc;

pub async fn new(socket: &Arc<Socket>, config: I2cConfig) -> Result<I2cBus, I2cBusModuleError> {
    let identifier = match &config {
        I2cConfig::I2c0 { .. } => I2cBusNumber::I2c0,
        I2cConfig::I2c1 { .. } => I2cBusNumber::I2c1,
    };

    let command = Command::I2c_New { config };

    let response = socket.send(command).await??;

    match response {
        Response::I2c_New { result } => result,
        _ => Err(I2cBusModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }?;

    Ok(I2cBus {
        socket: socket.clone(),
        mutex: Mutex::new(()),
        bus_number: identifier,
    })
}

pub async fn drop(socket: &Socket, identifier: I2cBusNumber) -> Result<(), I2cBusModuleError> {
    let command = Command::I2c_Drop { identifier };

    let response = socket.send(command).await??;

    match response {
        Response::I2c_Drop { result } => result,
        _ => Err(I2cBusModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

#[inline]
pub async fn read(
    socket: &Arc<Socket>,
    mutex: &Mutex<()>,
    identifier: I2cBusNumber,
    address: u16,
    buffer: &mut [u8],
) -> Result<(), I2cBusModuleError> {
    let _mutex_guard = mutex.lock().await;

    read_inner(socket, identifier, address, buffer).await
}

#[inline]
pub async fn write(
    socket: &Arc<Socket>,
    mutex: &Mutex<()>,
    identifier: I2cBusNumber,
    address: u16,
    bytes: &[u8],
) -> Result<(), I2cBusModuleError> {
    let _mutex_guard = mutex.lock().await;

    write_inner(socket, identifier, address, bytes).await
}

#[inline]
pub async fn write_read(
    socket: &Arc<Socket>,
    mutex: &Mutex<()>,
    identifier: I2cBusNumber,
    address: u16,
    bytes: &[u8],
    buffer: &mut [u8],
) -> Result<(), I2cBusModuleError> {
    let _mutex_guard = mutex.lock().await;

    write_read_inner(socket, identifier, address, bytes, buffer).await
}

#[inline]
#[cfg(feature = "embedded-hal")]
pub async fn transaction(
    socket: &Arc<Socket>,
    mutex: &Mutex<()>,
    identifier: I2cBusNumber,
    address: u16,
    operations: &mut [embedded_hal::i2c::Operation<'_>],
) -> Result<(), I2cBusModuleError> {
    let _mutex_guard = mutex.lock().await;

    for operation in operations {
        match operation {
            embedded_hal::i2c::Operation::Read(x) => {
                read_inner(socket, identifier, address, x).await?;
            }
            embedded_hal::i2c::Operation::Write(x) => {
                write_inner(socket, identifier, address, x).await?;
            }
        }
    }

    Ok(())
}

async fn read_inner(
    socket: &Arc<Socket>,
    identifier: I2cBusNumber,
    address: u16,
    buffer: &mut [u8],
) -> Result<(), I2cBusModuleError> {
    if buffer.len() <= BUS_BUFFER_SIZE {
        let command = Command::I2c_ReadSingle {
            identifier,
            address,
            buffer_size: buffer.len() as u16,
        };

        let response = socket.send(command).await??;

        let bus_buffer = match response {
            Response::I2c_ReadSingle { result } => result,
            _ => Err(I2cBusModuleError::from(FatalError::from(
                ProtocolError::ReceivedWrongResponse,
            ))),
        }?;

        for (to, from) in buffer.iter_mut().zip(bus_buffer) {
            *to = from;
        }

        Ok(())
    } else {
        let chunks = buffer.chunks_mut(BUS_BUFFER_SIZE).enumerate();
        let chunks_count = chunks.len();

        {
            let command = Command::I2c_StartReadChunked {
                identifier,
                address,
                chunks_count: chunks_count as u32,
            };

            let response = socket.send(command).await??;

            match response {
                Response::I2c_StartReadChunked { result } => result,
                _ => Err(I2cBusModuleError::from(FatalError::from(
                    ProtocolError::ReceivedWrongResponse,
                ))),
            }?;
        }

        let chunked_auto_closeable = ChunkedAutoCloseable::new(socket, identifier, ChunkedModeType::Read);

        for (chunk_index, chunk) in chunks {
            let command = Command::I2c_ReadChunk {
                identifier,
                buffer_size: chunk.len() as u16,
                chunk_index: chunk_index as u32,
            };

            let response = socket.send(command).await??;

            let bus_buffer = match response {
                Response::I2c_ReadSingle { result } => result,
                _ => Err(I2cBusModuleError::from(FatalError::from(
                    ProtocolError::ReceivedWrongResponse,
                ))),
            }?;

            for (to, from) in chunk.iter_mut().zip(bus_buffer) {
                *to = from;
            }
        }

        chunked_auto_closeable.drop_async().await?;

        Ok(())
    }
}

async fn write_inner(
    socket: &Arc<Socket>,
    identifier: I2cBusNumber,
    address: u16,
    bytes: &[u8],
) -> Result<(), I2cBusModuleError> {
    if bytes.len() <= BUS_BUFFER_SIZE {
        let command = Command::I2c_WriteSingle {
            identifier,
            address,
            bytes: BusBuffer::from_slice(bytes).unwrap(),
        };

        let response = socket.send(command).await??;

        match response {
            Response::I2c_WriteSingle { result } => result,
            _ => Err(I2cBusModuleError::from(FatalError::from(
                ProtocolError::ReceivedWrongResponse,
            ))),
        }?;

        Ok(())
    } else {
        let chunks = bytes.chunks(BUS_BUFFER_SIZE).enumerate();
        let chunks_count = chunks.len();

        {
            let command = Command::I2c_StartWriteChunked {
                identifier,
                address,
                chunks_count: chunks_count as u32,
            };

            let response = socket.send(command).await??;

            match response {
                Response::I2c_StartWriteChunked { result } => result,
                _ => Err(I2cBusModuleError::from(FatalError::from(
                    ProtocolError::ReceivedWrongResponse,
                ))),
            }?;
        }

        let chunked_auto_closeable = ChunkedAutoCloseable::new(socket, identifier, ChunkedModeType::Write);

        for (chunk_index, chunk) in chunks {
            let command = Command::I2c_WriteChunk {
                identifier,
                bytes: BusBuffer::from_slice(chunk).unwrap(),
                chunk_index: chunk_index as u32,
            };

            let response = socket.send(command).await??;

            match response {
                Response::I2c_WriteChunk { result } => result,
                _ => Err(I2cBusModuleError::from(FatalError::from(
                    ProtocolError::ReceivedWrongResponse,
                ))),
            }?;
        }

        chunked_auto_closeable.drop_async().await?;

        Ok(())
    }
}

async fn write_read_inner(
    socket: &Arc<Socket>,
    identifier: I2cBusNumber,
    address: u16,
    bytes: &[u8],
    buffer: &mut [u8],
) -> Result<(), I2cBusModuleError> {
    if buffer.len() <= BUS_BUFFER_SIZE && bytes.len() <= BUS_BUFFER_SIZE {
        let command = Command::I2c_WriteReadSingle {
            identifier,
            address,
            bytes: BusBuffer::from_slice(bytes).unwrap(),
            buffer_size: buffer.len() as u16,
        };

        let response = socket.send(command).await??;

        let bus_buffer = match response {
            Response::I2c_WriteReadSingle { result } => result,
            _ => Err(I2cBusModuleError::from(FatalError::from(
                ProtocolError::ReceivedWrongResponse,
            ))),
        }?;

        for (to, from) in buffer.iter_mut().zip(bus_buffer) {
            *to = from;
        }

        Ok(())
    } else {
        write_inner(socket, identifier, address, bytes).await?;
        read_inner(socket, identifier, address, buffer).await?;

        Ok(())
    }
}

#[derive(Copy, Clone)]
enum ChunkedModeType {
    Write,
    Read,
}

struct ChunkedAutoCloseable {
    socket: Option<Arc<Socket>>,
    mode: ChunkedModeType,
    bus: I2cBusNumber,
}

impl ChunkedAutoCloseable {
    pub fn new(socket: &Arc<Socket>, identifier: I2cBusNumber, mode: ChunkedModeType) -> ChunkedAutoCloseable {
        ChunkedAutoCloseable {
            socket: Some(socket.clone()),
            mode,
            bus: identifier,
        }
    }

    pub async fn drop_async(mut self) -> Result<(), I2cBusModuleError> {
        match self.socket.take() {
            None => Ok(()),
            Some(socket) => drop_async_inner(socket, self.mode, self.bus).await,
        }
    }
}

impl Drop for ChunkedAutoCloseable {
    fn drop(&mut self) {
        match self.socket.take() {
            None => {}
            Some(socket) => {
                let mode = self.mode;
                let bus = self.bus;

                block_on(async move { _ = drop_async_inner(socket, mode, bus).await })
            }
        }
    }
}

async fn drop_async_inner(
    socket: Arc<Socket>,
    mode: ChunkedModeType,
    identifier: I2cBusNumber,
) -> Result<(), I2cBusModuleError> {
    match mode {
        ChunkedModeType::Write => {
            let command = Command::I2c_StopWriteChunked { identifier };

            let response = socket.send(command).await??;

            match response {
                Response::I2c_StopWriteChunked { result } => result,
                _ => Err(I2cBusModuleError::from(FatalError::from(
                    ProtocolError::ReceivedWrongResponse,
                ))),
            }?;
        }
        ChunkedModeType::Read => {
            let command = Command::I2c_StopReadChunked { identifier };

            let response = socket.send(command).await??;

            match response {
                Response::I2c_StopReadChunked { result } => result,
                _ => Err(I2cBusModuleError::from(FatalError::from(
                    ProtocolError::ReceivedWrongResponse,
                ))),
            }?;
        }
    }

    Ok(())
}
