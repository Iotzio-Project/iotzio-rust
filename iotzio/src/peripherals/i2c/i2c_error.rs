use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

/// I2C bus error.
#[non_exhaustive]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Error))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum I2cError {
    #[error("Abort - a bus operation was not acknowledged, e.g. due to the addressed device not being available on the bus or the device not being ready to process requests at the moment.")]
    AbortNoAcknowledge,
    #[error("Abort - the arbitration was lost, e.g. electrical problems with the clock signal.")]
    AbortArbitrationLoss,
    #[error("Abort - transmit ended with data still in fifo.")]
    AbortTxNotEmpty { length: u16 },
    #[error("Abort - other reason.")]
    AbortOther,
    #[error("Passed in a read buffer that was 0 length.")]
    InvalidReadBufferLength,
    #[error("Passed in a write buffer that was 0 length.")]
    InvalidWriteBufferLength,
    #[error("Target i2c address {address} is out of range.")]
    AddressOutOfRange { address: u16 },
    #[error("Target i2c address {address} is reserved.")]
    AddressReserved { address: u16 },
}
