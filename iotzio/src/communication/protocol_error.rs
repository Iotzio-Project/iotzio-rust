use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

#[non_exhaustive]
#[cfg_attr(any(feature = "_uniffi-blocking", feature = "_uniffi-async"), derive(uniffi::Error))]
#[cfg_attr(feature = "_defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Error, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProtocolError {
    #[error("The packet is too small.")]
    PacketTooSmall,
    #[error("Found no Report ID to fit buffer size.")]
    ErrorSelectingReportId,
    #[error("Received wrong response.")]
    ReceivedWrongResponse,
    #[error("Received command error for a command that can never fail in that way.")]
    ReceivedImpossibleCommandError,
    #[error("Received a command that should never show up in that scenario.")]
    ReceivedImpossibleCommand,

    // Postcard Errors
    #[error("This is a feature that PostCard will never implement.")]
    WontImplement,
    #[error("This is a feature that Postcard intends to support, but does not yet.")]
    NotYetImplemented,
    #[error("The serialize buffer is full.")]
    SerializeBufferFull,
    #[error("The length of a sequence must be known.")]
    SerializeSeqLengthUnknown,
    #[error("Hit the end of buffer, expected more data.")]
    DeserializeUnexpectedEnd,
    #[error("Found a varint that didn't terminate. Is the usize too big for this platform?")]
    DeserializeBadVarint,
    #[error("Found a bool that wasn't 0 or 1.")]
    DeserializeBadBool,
    #[error("Found an invalid unicode char.")]
    DeserializeBadChar,
    #[error("Tried to parse invalid utf-8.")]
    DeserializeBadUtf8,
    #[error("Found an Option discriminant that wasn't 0 or 1.")]
    DeserializeBadOption,
    #[error("Found an enum discriminant that was > u32::max_value().")]
    DeserializeBadEnum,
    #[error("The original data was not well encoded.")]
    DeserializeBadEncoding,
    #[error("Bad CRC while deserializing.")]
    DeserializeBadCrc,
    #[error("Serde Serialization Error.")]
    SerdeSerCustom,
    #[error("Serde Deserialization Error.")]
    SerdeDeCustom,
    #[error("Error while processing `collect_str` during serialization.")]
    CollectStrError,

    // Postcard Exhaustive
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
