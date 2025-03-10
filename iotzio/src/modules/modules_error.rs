use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

/// Error that could be raised by every module.
#[non_exhaustive]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModuleError {
    /// Unknown command. Update your Iotzio Firmware.
    #[error("Unknown command. Update your Iotzio Firmware.")]
    UnknownCommand,

    /// A license is missing to use this module.
    #[error("A license is missing to use this module.")]
    UnlicensedModule,

    /// A required device peripheral is blocked by another module.
    #[error("A required device peripheral is blocked by another module.")]
    PeripheralBlockedByAnotherModule,

    /// Parallel access to this module interrupted the current command.
    #[error("Parallel access to this module interrupted the current command.")]
    ModuleCommandInterrupted,

    /// Module storage exhausted on device, unable to handle additional module instances.
    #[error("Module storage exhausted on device, unable to handle additional module instances.")]
    ModuleStorageExhausted,

    /// The requested module instance was no longer found on the device. Maybe it was previously disabled.
    #[error("The requested module instance was no longer found on the device. Maybe it was previously disabled.")]
    ModuleInstanceNotFound,
}
