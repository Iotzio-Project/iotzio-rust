use crate::communication::{BoardInfo, Command, FatalError, ProtocolError, Response, Version};
use crate::iotzio::iotzio_info::IotzioInfo;
use crate::iotzio::USB_PRODUCT_NAME_PREFIX;
use crate::socket::Socket;
use crate::{backend, InitializationError, Iotzio};

pub async fn list_connected_boards() -> Result<Vec<IotzioInfo>, InitializationError> {
    backend::list_connected_boards(parse_version)
        .await
        .map_err(|x| InitializationError::DeviceOpenError {
            error_message: format!("Error listing connected Iotzio boards: {0}", x),
        })
}

fn parse_version(product_name: &str) -> Option<Version> {
    if product_name.starts_with(USB_PRODUCT_NAME_PREFIX) {
        let version_part = &product_name[7..];

        let parts: Vec<&str> = version_part.split('.').collect();

        if parts.len() == 3 {
            if let (Ok(major), Ok(minor), Ok(patch)) = (
                parts[0].parse::<u16>(),
                parts[1].parse::<u16>(),
                parts[2].parse::<u16>(),
            ) {
                return Some(Version { major, minor, patch });
            }
        }
    }
    None
}

pub async fn new_iotzio(iotzio_info: &IotzioInfo) -> Result<Iotzio, InitializationError> {
    let socket = Socket::new(&iotzio_info).await?;

    let board_info = initialize_board(&socket).await?;

    Ok(Iotzio {
        socket: socket.into(),
        board_info,
    })
}

#[inline]
async fn initialize_board(socket: &Socket) -> Result<BoardInfo, FatalError> {
    let command = Command::Initialize;

    let response = socket.send(command).await?.map_err(|_| FatalError::HostProtocolError {
        error: ProtocolError::ReceivedImpossibleCommandError,
    })?;

    match response {
        Response::Initialize { board_info } => Ok(board_info),
        _ => Err(FatalError::HostProtocolError {
            error: ProtocolError::ReceivedWrongResponse,
        }),
    }
}
