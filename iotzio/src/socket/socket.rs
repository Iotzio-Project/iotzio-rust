use crate::backend::{DeviceReader, DeviceWriter};
use crate::communication::{Command, FatalError, Response};
use crate::modules::ModuleError;
use crate::socket::{socket_service, RuntimeIdentifier};
use crate::{InitializationError, IotzioInfo};
use async_oneshot::Sender;
use async_std::sync::Mutex;
use std::sync::atomic::AtomicU32;

#[derive(Debug)]
pub struct Socket {
    pub(crate) output: Mutex<SocketOutput>,
    pub(crate) input: Mutex<SocketInput>,
    pub(crate) packet_counter: AtomicU32,
    pub(crate) input_queue_mutex: Mutex<Vec<(u32, Sender<Result<Result<Response, ModuleError>, FatalError>>)>>,
    pub(crate) runtime_identifier: RuntimeIdentifier,
}

#[derive(Debug)]
pub struct SocketOutput {
    pub(crate) writer: DeviceWriter,
    pub(crate) buffer: Box<[u8]>,
    pub(crate) reports: Vec<(u8, usize)>,
}

#[derive(Debug)]
pub struct SocketInput {
    pub(crate) reader: DeviceReader,
    pub(crate) buffer: Box<[u8]>,
}

impl Socket {
    #[inline]
    pub async fn new(iotzio_info: &IotzioInfo) -> Result<Socket, InitializationError> {
        socket_service::new_socket(iotzio_info).await
    }

    #[inline]
    pub async fn send(&self, command: Command) -> Result<Result<Response, ModuleError>, FatalError> {
        socket_service::send_command(self, command).await
    }
}
