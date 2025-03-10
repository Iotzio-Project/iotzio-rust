use crate::communication::BoardInfo;
use crate::modules;
use crate::peripherals::gpio::Level;
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

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Response {
    Initialize {
        board_info: BoardInfo,
    },
    InputPin_New {
        result: Result<(), modules::input_pin::InputPinModuleError>,
    },
    InputPin_Drop {
        result: Result<(), modules::input_pin::InputPinModuleError>,
    },
    InputPin_GetLevel {
        result: Result<Level, modules::input_pin::InputPinModuleError>,
    },
    InputPin_WaitForSignal {
        result: Result<modules::input_pin::SignalTypeResponse, modules::input_pin::InputPinModuleError>,
    },
    OutputPin_New {
        result: Result<(), modules::output_pin::OutputPinModuleError>,
    },
    OutputPin_Drop {
        result: Result<(), modules::output_pin::OutputPinModuleError>,
    },
    OutputPin_SetLevel {
        result: Result<(), modules::output_pin::OutputPinModuleError>,
    },
    I2c_New {
        result: Result<(), modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_Drop {
        result: Result<(), modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_ReadSingle {
        result: Result<BusBuffer, modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_StartReadChunked {
        result: Result<(), modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_ReadChunk {
        result: Result<BusBuffer, modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_StopReadChunked {
        result: Result<(), modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_WriteSingle {
        result: Result<(), modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_StartWriteChunked {
        result: Result<(), modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_WriteChunk {
        result: Result<(), modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_StopWriteChunked {
        result: Result<(), modules::i2c_bus::I2cBusModuleError>,
    },
    I2c_WriteReadSingle {
        result: Result<BusBuffer, modules::i2c_bus::I2cBusModuleError>,
    },
}
