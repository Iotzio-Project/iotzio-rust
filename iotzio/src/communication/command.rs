use crate::modules;
use crate::peripherals::gpio::{Drive, GpioPin, Level, Pull, SlewRate};
use crate::peripherals::i2c::I2cIdentifier;
use crate::peripherals::BusBuffer;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "_host")] {
        use std::fmt;
    }
    else {
        use core::fmt;
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Command {
    Initialize,
    InputPin_New {
        pin: GpioPin,
        pull_setting: Pull,
        hysteresis: bool,
    },
    InputPin_Drop {
        pin: GpioPin,
    },
    InputPin_GetLevel {
        pin: GpioPin,
    },
    InputPin_WaitForSignal {
        pin: GpioPin,
        signal_type: modules::input_pin::SignalTypeRequest,
    },
    OutputPin_New {
        pin: GpioPin,
        initial_level: Level,
        drive_strength: Drive,
        slew_rate: SlewRate,
    },
    OutputPin_Drop {
        pin: GpioPin,
    },
    OutputPin_SetLevel {
        pin: GpioPin,
        level: Level,
    },
    I2c_New {
        config: modules::i2c_bus::I2cConfig,
    },
    I2c_Drop {
        identifier: I2cIdentifier,
    },
    I2c_ReadSingle {
        identifier: I2cIdentifier,
        address: u16,
        buffer_size: u16,
    },
    I2c_StartReadChunked {
        identifier: I2cIdentifier,
        address: u16,
        chunks_count: u32,
    },
    I2c_ReadChunk {
        identifier: I2cIdentifier,
        buffer_size: u16,
        chunk_index: u32,
    },
    I2c_StopReadChunked {
        identifier: I2cIdentifier,
    },
    I2c_WriteSingle {
        identifier: I2cIdentifier,
        address: u16,
        bytes: BusBuffer,
    },
    I2c_StartWriteChunked {
        identifier: I2cIdentifier,
        address: u16,
        chunks_count: u32,
    },
    I2c_WriteChunk {
        identifier: I2cIdentifier,
        bytes: BusBuffer,
        chunk_index: u32,
    },
    I2c_StopWriteChunked {
        identifier: I2cIdentifier,
    },
    I2c_WriteReadSingle {
        identifier: I2cIdentifier,
        address: u16,
        bytes: BusBuffer,
        buffer_size: u16,
    },
}

impl Command {
    pub fn id(&self) -> u16 {
        let value = match self {
            Command::Initialize => 0,
            Command::InputPin_New { .. } => 1,
            Command::InputPin_Drop { .. } => 2,
            Command::InputPin_GetLevel { .. } => 3,
            Command::InputPin_WaitForSignal { .. } => 4,
            Command::OutputPin_New { .. } => 5,
            Command::OutputPin_Drop { .. } => 6,
            Command::OutputPin_SetLevel { .. } => 7,
            Command::I2c_New { .. } => 8,
            Command::I2c_Drop { .. } => 9,
            Command::I2c_ReadSingle { .. } => 10,
            Command::I2c_StartReadChunked { .. } => 11,
            Command::I2c_ReadChunk { .. } => 12,
            Command::I2c_StopReadChunked { .. } => 13,
            Command::I2c_WriteSingle { .. } => 14,
            Command::I2c_StartWriteChunked { .. } => 15,
            Command::I2c_WriteChunk { .. } => 16,
            Command::I2c_StopWriteChunked { .. } => 17,
            Command::I2c_WriteReadSingle { .. } => 18,
        };

        debug_assert!(value < COMMAND_COUNT, "Command count not updated.");

        value
    }
}

pub const COMMAND_COUNT: u16 = 19;
