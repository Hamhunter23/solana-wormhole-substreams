// @generated
/// This message represents an NFT transfer event through the Wormhole NFT Bridge
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftTransfer {
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
    /// Data specific to NFT transfers
    ///
    /// The NFT contract address on source chain
    #[prost(string, tag="6")]
    pub nft_address: ::prost::alloc::string::String,
    /// Source chain ID
    #[prost(string, tag="7")]
    pub nft_chain: ::prost::alloc::string::String,
    /// Recipient address on target chain (serialized based on target chain)
    #[prost(string, tag="8")]
    pub to_address: ::prost::alloc::string::String,
    /// Destination chain ID
    #[prost(string, tag="9")]
    pub to_chain: ::prost::alloc::string::String,
    /// Sender address (Solana account in base58)
    #[prost(string, tag="10")]
    pub from_address: ::prost::alloc::string::String,
    /// NFT token ID
    #[prost(uint64, tag="11")]
    pub token_id: u64,
    /// NFT token URI
    #[prost(string, tag="12")]
    pub uri: ::prost::alloc::string::String,
    /// NFT name
    #[prost(string, tag="13")]
    pub name: ::prost::alloc::string::String,
    /// NFT collection symbol
    #[prost(string, tag="14")]
    pub symbol: ::prost::alloc::string::String,
    /// Sequence number (for ordering)
    #[prost(uint64, tag="15")]
    pub sequence: u64,
    /// Any additional payload data (base64 encoded)
    #[prost(string, tag="16")]
    pub payload: ::prost::alloc::string::String,
}
/// Container for multiple NFT transfer events
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftTransfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<NftTransfer>,
}
// @@protoc_insertion_point(module)
