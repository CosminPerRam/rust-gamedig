use core::fmt;
use std::fmt::Formatter;

/// Result of Type and GDError.
pub type GDResult<T> = Result<T, GDError>;

/// GameDigError, every error you can encounter using the library.
#[derive(Debug, Clone)]
pub enum GDError {
    /// The received packet was bigger than the buffer size.
    PacketOverflow(String),
    /// The received packet was shorter than the expected one.
    PacketUnderflow(String),
    /// The received packet was badly formatted.
    PacketBad(String),
    /// Couldn't send the packet.
    PacketSend(String),
    /// Couldn't send the receive.
    PacketReceive(String),
    /// Couldn't decompress data.
    Decompress(String),
    /// Unknown cast while translating a value to an enum.
    UnknownEnumCast,
    /// The server queried is not from the queried game.
    BadGame(String),
    /// Problems occurred while dns resolving.
    DnsResolve(String),
}

impl fmt::Display for GDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GDError::PacketOverflow(details) => write!(f, "Packet overflow: {details}"),
            GDError::PacketUnderflow(details) => write!(f, "Packet underflow: {details}"),
            GDError::PacketBad(details) => write!(f, "Packet bad: {details}"),
            GDError::PacketSend(details) => write!(f, "Couldn't send a packet: {details}"),
            GDError::PacketReceive(details) => write!(f, "Couldn't receive a packet: {details}"),
            GDError::Decompress(details) => write!(f, "Couldn't decompress data: {details}"),
            GDError::UnknownEnumCast => write!(f, "Unknown enum cast encountered."),
            GDError::BadGame(details) => write!(f, "Queried another game that the supposed one: {details}"),
            GDError::DnsResolve(details) => write!(f, "DNS Resolve: {details}"),
        }
    }
}
