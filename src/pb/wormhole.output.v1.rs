// @generated
/// This message represents the data we extract for each Wormhole MessagePublication event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePublication {
    /// Contextual data from the Solana transaction/block
    ///
    /// Base58 encoded transaction signature
    #[prost(string, tag="1")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_slot: u64,
    /// In seconds
    #[prost(int64, tag="3")]
    pub block_timestamp: i64,
    /// Index of the top-level instruction
    #[prost(uint32, tag="4")]
    pub instruction_index: u32,
    /// Index within inner instructions (0 if top-level)
    #[prost(uint32, tag="5")]
    pub inner_instruction_index: u32,
    /// Data directly from the Wormhole MessagePublication event
    #[prost(uint32, tag="6")]
    pub nonce: u32,
    /// The raw message payload
    #[prost(bytes="vec", tag="7")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    /// Base58 encoded emitter address
    #[prost(string, tag="8")]
    pub emitter_account: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub sequence: u64,
    /// Mapped from u8
    #[prost(uint32, tag="10")]
    pub consistency_level: u32,
    /// Timestamp from the event itself (might be same as block, depends on program)
    #[prost(uint64, tag="11")]
    pub event_timestamp: u64,
}
/// A container for multiple MessagePublication events per block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePublications {
    #[prost(message, repeated, tag="1")]
    pub publications: ::prost::alloc::vec::Vec<MessagePublication>,
}
// @@protoc_insertion_point(module)
