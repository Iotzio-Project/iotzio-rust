use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

/// I2C bus error.
#[non_exhaustive]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2cError {
    /// Abort - a bus operation was not acknowledged, e.g. due to the addressed device not being available on the bus or the device not being ready to process requests at the moment.
    #[error("Abort - a bus operation was not acknowledged, e.g. due to the addressed device not being available on the bus or the device not being ready to process requests at the moment.")]
    AbortNoAcknowledge,

    /// Abort - the arbitration was lost, e.g. electrical problems with the clock signal.
    #[error("Abort - the arbitration was lost, e.g. electrical problems with the clock signal.")]
    AbortArbitrationLoss,

    /// Abort - transmit ended with data still in fifo.
    #[error("Abort - transmit ended with data still in fifo.")]
    AbortTxNotEmpty { length: u16 },

    /// Abort - other reason.
    #[error("Abort - other reason.")]
    AbortOther,

    /// Passed in a read buffer that was 0 length.
    #[error("Passed in a read buffer that was 0 length.")]
    InvalidReadBufferLength,

    /// Passed in a write buffer that was 0 length.
    #[error("Passed in a write buffer that was 0 length.")]
    InvalidWriteBufferLength,

    /// Target i2c address is out of range.
    #[error("Target i2c address {address} is out of range.")]
    AddressOutOfRange { address: u16 },

    /// Target i2c address is reserved.
    #[error("Target i2c address {address} is reserved.")]
    AddressReserved { address: u16 },
}
