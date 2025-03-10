use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

/// A protocol error indicates an internal error in the communication protocol.
#[non_exhaustive]
#[cfg_attr(any(feature = "_ffi-blocking", feature = "_ffi-async"), derive(uniffi::Error))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Error, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProtocolError {
    /// The packet is too small.
    #[error("The packet is too small.")]
    PacketTooSmall,

    /// Found no Report ID to fit buffer size.
    #[error("Found no Report ID to fit buffer size.")]
    ErrorSelectingReportId,

    /// Received wrong response.
    #[error("Received wrong response.")]
    ReceivedWrongResponse,

    /// Received command error for a command that can never fail in that way.
    #[error("Received command error for a command that can never fail in that way.")]
    ReceivedImpossibleCommandError,

    /// Received a command that should never show up in that scenario.
    #[error("Received a command that should never show up in that scenario.")]
    ReceivedImpossibleCommand,

    // Postcard Errors
    /// This is a feature that PostCard will never implement.
    #[error("This is a feature that PostCard will never implement.")]
    WontImplement,

    /// This is a feature that Postcard intends to support, but does not yet.
    #[error("This is a feature that Postcard intends to support, but does not yet.")]
    NotYetImplemented,

    /// The serialize buffer is full.
    #[error("The serialize buffer is full.")]
    SerializeBufferFull,

    /// The length of a sequence must be known.
    #[error("The length of a sequence must be known.")]
    SerializeSeqLengthUnknown,

    /// Hit the end of buffer, expected more data.
    #[error("Hit the end of buffer, expected more data.")]
    DeserializeUnexpectedEnd,

    /// Found a varint that didn't terminate. Is the usize too big for this platform?
    #[error("Found a varint that didn't terminate. Is the usize too big for this platform?")]
    DeserializeBadVarint,

    /// Found a bool that wasn't 0 or 1.
    #[error("Found a bool that wasn't 0 or 1.")]
    DeserializeBadBool,

    /// Found an invalid unicode char.
    #[error("Found an invalid unicode char.")]
    DeserializeBadChar,

    /// Tried to parse invalid utf-8.
    #[error("Tried to parse invalid utf-8.")]
    DeserializeBadUtf8,

    /// Found an Option discriminant that wasn't 0 or 1.
    #[error("Found an Option discriminant that wasn't 0 or 1.")]
    DeserializeBadOption,

    /// Found an enum discriminant that was > u32::max_value().
    #[error("Found an enum discriminant that was > u32::max_value().")]
    DeserializeBadEnum,

    /// The original data was not well encoded.
    #[error("The original data was not well encoded.")]
    DeserializeBadEncoding,

    /// Bad CRC while deserializing.
    #[error("Bad CRC while deserializing.")]
    DeserializeBadCrc,

    /// Serde Serialization Error.
    #[error("Serde Serialization Error.")]
    SerdeSerCustom,

    /// Serde Deserialization Error.
    #[error("Serde Deserialization Error.")]
    SerdeDeCustom,

    /// Error while processing `collect_str` during serialization.
    #[error("Error while processing `collect_str` during serialization.")]
    CollectStrError,

    // Postcard Exhaustive
    /// Unknown error.
    #[error("Unknown error.")]
    Unknown,
}

impl From<postcard::Error> for ProtocolError {
    fn from(value: postcard::Error) -> Self {
        match value {
            postcard::Error::WontImplement => ProtocolError::WontImplement,
            postcard::Error::NotYetImplemented => ProtocolError::NotYetImplemented,
            postcard::Error::SerializeBufferFull => ProtocolError::SerializeBufferFull,
            postcard::Error::SerializeSeqLengthUnknown => ProtocolError::SerializeSeqLengthUnknown,
            postcard::Error::DeserializeUnexpectedEnd => ProtocolError::DeserializeUnexpectedEnd,
            postcard::Error::DeserializeBadVarint => ProtocolError::DeserializeBadVarint,
            postcard::Error::DeserializeBadBool => ProtocolError::DeserializeBadBool,
            postcard::Error::DeserializeBadChar => ProtocolError::DeserializeBadChar,
            postcard::Error::DeserializeBadUtf8 => ProtocolError::DeserializeBadUtf8,
            postcard::Error::DeserializeBadOption => ProtocolError::DeserializeBadOption,
            postcard::Error::DeserializeBadEnum => ProtocolError::DeserializeBadEnum,
            postcard::Error::DeserializeBadEncoding => ProtocolError::DeserializeBadEncoding,
            postcard::Error::DeserializeBadCrc => ProtocolError::DeserializeBadCrc,
            postcard::Error::SerdeSerCustom => ProtocolError::SerdeSerCustom,
            postcard::Error::SerdeDeCustom => ProtocolError::SerdeDeCustom,
            postcard::Error::CollectStrError => ProtocolError::CollectStrError,
            _ => ProtocolError::Unknown,
        }
    }
}
