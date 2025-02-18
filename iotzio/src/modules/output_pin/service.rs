use crate::communication::{Command, FatalError, ProtocolError, Response};
use crate::modules::output_pin::{OutputPin, OutputPinModuleError};
use crate::peripherals::gpio::{Drive, GpioPin, Level, SlewRate};
use crate::socket::Socket;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

pub async fn new(
    socket: &Arc<Socket>,
    pin: GpioPin,
    initial_level: Level,
    drive_strength: Drive,
    slew_rate: SlewRate,
) -> Result<OutputPin, OutputPinModuleError> {
    let command = Command::OutputPin_New {
        pin,
        initial_level,
        drive_strength,
        slew_rate,
    };

    let response = socket.send(command).await??;

    match response {
        Response::OutputPin_New { result } => result,
        _ => Err(OutputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }?;

    Ok(OutputPin {
        socket: socket.clone(),
        level: initial_level.into(),
        pin,
        drive_strength,
        slew_rate,
    })
}

pub async fn drop(socket: &Socket, pin: GpioPin) -> Result<(), OutputPinModuleError> {
    let command = Command::OutputPin_Drop { pin };

    let response = socket.send(command).await??;

    match response {
        Response::OutputPin_Drop { result } => result,
        _ => Err(OutputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn set_level(
    socket: &Socket,
    level_mutex: &Mutex<Level>,
    pin: GpioPin,
    level: Level,
) -> Result<(), OutputPinModuleError> {
    {
        let command = Command::OutputPin_SetLevel { pin, level };

        let response = socket.send(command).await??;

        match response {
            Response::OutputPin_SetLevel { result } => result,
            _ => Err(OutputPinModuleError::from(FatalError::from(
                ProtocolError::ReceivedWrongResponse,
            ))),
        }
    }
    .map(|_| {
        *level_mutex.lock().unwrap().deref_mut() = level;
    })
}
