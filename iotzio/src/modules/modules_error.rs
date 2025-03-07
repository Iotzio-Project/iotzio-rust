use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

#[non_exhaustive]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Error, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModuleError {
    #[error("Unknown command. Update your Iotzio Firmware.")]
    UnknownCommand,
    #[error("A license is missing to use this module.")]
    UnlicensedModule,
    #[error("A required peripheral is blocked by another module.")]
    PeripheralBlockedByAnotherModule,
    #[error("A module command forced a cancellation of this task.")]
    ModuleTaskCancelled,
    #[error("Module storage exhausted, unable to handle additional module instances.")]
    ModuleStorageExhausted,
    #[error("The requested module instance was no longer found. Maybe is was previously disabled.")]
    ModuleInstanceNotFound,
}
