use crate::communication::{Command, FatalError, ProtocolError, Response};
use crate::modules::input_pin::{InputPin, InputPinModuleError, SignalTypeRequest, SignalTypeResponse};
use crate::peripherals::gpio::{GpioPin, Level, Pull};
use crate::socket::Socket;
use std::sync::Arc;
use std::time::Duration;

pub async fn new(
    socket: &Arc<Socket>,
    pin: GpioPin,
    pull_setting: Pull,
    hysteresis: bool,
) -> Result<InputPin, InputPinModuleError> {
    let command = Command::InputPin_New {
        pin,
        pull_setting,
        hysteresis,
    };

    let response = socket.send(command).await??;

    match response {
        Response::InputPin_New { result } => result,
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }?;

    Ok(InputPin {
        socket: socket.clone(),
        pin,
        pull_setting,
        hysteresis,
    })
}

pub async fn drop(socket: &Socket, pin: GpioPin) -> Result<(), InputPinModuleError> {
    let command = Command::InputPin_Drop { pin };

    let response = socket.send(command).await??;

    match response {
        Response::InputPin_Drop { result } => result,
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn is_high(socket: &Socket, pin: GpioPin) -> Result<bool, InputPinModuleError> {
    get_level(socket, pin).await.map(|x| x == Level::High)
}

pub async fn is_low(socket: &Socket, pin: GpioPin) -> Result<bool, InputPinModuleError> {
    get_level(socket, pin).await.map(|x| x == Level::Low)
}

pub async fn get_level(socket: &Socket, pin: GpioPin) -> Result<Level, InputPinModuleError> {
    let command = Command::InputPin_GetLevel { pin };

    let response = socket.send(command).await??;

    match response {
        Response::InputPin_GetLevel { result } => result,
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn wait_for_high(socket: &Socket, pin: GpioPin) -> Result<(), InputPinModuleError> {
    match wait_for(socket, pin, SignalTypeRequest::High).await? {
        SignalTypeResponse::High => Ok(()),
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn wait_for_low(socket: &Socket, pin: GpioPin) -> Result<(), InputPinModuleError> {
    match wait_for(socket, pin, SignalTypeRequest::Low).await? {
        SignalTypeResponse::Low => Ok(()),
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn wait_for_rising_edge(socket: &Socket, pin: GpioPin) -> Result<(), InputPinModuleError> {
    match wait_for(socket, pin, SignalTypeRequest::RisingEdge).await? {
        SignalTypeResponse::RisingEdge => Ok(()),
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn wait_for_falling_edge(socket: &Socket, pin: GpioPin) -> Result<(), InputPinModuleError> {
    match wait_for(socket, pin, SignalTypeRequest::FallingEdge).await? {
        SignalTypeResponse::FallingEdge => Ok(()),
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn wait_for_any_edge(socket: &Socket, pin: GpioPin) -> Result<(), InputPinModuleError> {
    match wait_for(socket, pin, SignalTypeRequest::AnyEdge).await? {
        SignalTypeResponse::AnyEdge => Ok(()),
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn wait_for_high_pulse(socket: &Socket, pin: GpioPin) -> Result<Duration, InputPinModuleError> {
    match wait_for(socket, pin, SignalTypeRequest::HighPulse).await? {
        SignalTypeResponse::HighPulse(x) => Ok(x),
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn wait_for_low_pulse(socket: &Socket, pin: GpioPin) -> Result<Duration, InputPinModuleError> {
    match wait_for(socket, pin, SignalTypeRequest::LowPulse).await? {
        SignalTypeResponse::LowPulse(x) => Ok(x),
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

pub async fn wait_for_any_pulse(socket: &Socket, pin: GpioPin) -> Result<Duration, InputPinModuleError> {
    match wait_for(socket, pin, SignalTypeRequest::AnyPulse).await? {
        SignalTypeResponse::AnyPulse(x) => Ok(x),
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}

#[inline]
async fn wait_for(
    socket: &Socket,
    pin: GpioPin,
    signal_type: SignalTypeRequest,
) -> Result<SignalTypeResponse, InputPinModuleError> {
    let command = Command::InputPin_WaitForSignal { pin, signal_type };

    let response = socket.send(command).await??;

    match response {
        Response::InputPin_WaitForSignal { result } => result,
        _ => Err(InputPinModuleError::from(FatalError::from(
            ProtocolError::ReceivedWrongResponse,
        ))),
    }
}
