// @generated
/// This message represents a token transfer event through the Wormhole Token Bridge
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenTransfer {
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
    /// Data specific to token transfers
    ///
    /// The token contract address on source chain
    #[prost(string, tag="6")]
    pub token_address: ::prost::alloc::string::String,
    /// Source chain ID
    #[prost(string, tag="7")]
    pub token_chain: ::prost::alloc::string::String,
    /// Recipient address on target chain (serialized based on target chain)
    #[prost(string, tag="8")]
    pub to_address: ::prost::alloc::string::String,
    /// Destination chain ID
    #[prost(string, tag="9")]
    pub to_chain: ::prost::alloc::string::String,
    /// Sender address (Solana account in base58)
    #[prost(string, tag="10")]
    pub from_address: ::prost::alloc::string::String,
    /// Transfer amount (in token's lowest denomination)
    #[prost(uint64, tag="11")]
    pub amount: u64,
    /// Fee amount (in token's lowest denomination)
    #[prost(uint64, tag="12")]
    pub fee: u64,
    /// Token symbol if available
    #[prost(string, tag="13")]
    pub token_symbol: ::prost::alloc::string::String,
    /// Token decimals if available
    #[prost(uint32, tag="14")]
    pub token_decimals: u32,
    /// Token name if available
    #[prost(string, tag="15")]
    pub token_name: ::prost::alloc::string::String,
    /// Sequence number (for ordering)
    #[prost(uint64, tag="16")]
    pub sequence: u64,
    /// Any additional payload data (base64 encoded)
    #[prost(string, tag="17")]
    pub payload: ::prost::alloc::string::String,
}
/// Container for multiple token transfer events
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenTransfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<TokenTransfer>,
}
// @@protoc_insertion_point(module)
