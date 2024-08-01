use crate::types::TranslucentPacket;

pub mod http;

/// A protocol that can be processed by Translucent should possess features that could be recognized
/// by a handler as soon as possible.
pub trait SupportedProtocol : TryInto<TranslucentPacket<Self>> {
    type PayloadType: Send;

    /// Determines if the head of a packet indicates a valid packet of this protocol. If so, this
    /// function returns a handler for it.
    fn from_packet(partial_packet: &[u8]) -> Option<Self>;

    /// Returns the exact size (if available) of remaining bytes in the current packet. Possible values:
    ///   None     => size of the packet cannot be determined,
    ///   Some(0)  => the received packet is complete,
    ///   Some(n)  => the received packet is incomplete, and the size of the remaining bytes is n.
    fn exact_size() -> Option<usize>;
}
