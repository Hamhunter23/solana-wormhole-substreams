// @generated
/// This message combines all Wormhole activities across different bridges
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WormholeActivity {
    /// Core bridge message publications
    #[prost(message, repeated, tag="1")]
    pub core_messages: ::prost::alloc::vec::Vec<super::super::output::v1::MessagePublication>,
    /// Token bridge transfers
    #[prost(message, repeated, tag="2")]
    pub token_transfers: ::prost::alloc::vec::Vec<super::super::token_bridge::v1::TokenTransfer>,
    /// NFT bridge transfers
    #[prost(message, repeated, tag="3")]
    pub nft_transfers: ::prost::alloc::vec::Vec<super::super::nft_bridge::v1::NftTransfer>,
    /// Various aggregated metrics
    ///
    /// Block timestamp
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    /// Total value locked in USD (if calculable)
    #[prost(uint64, tag="5")]
    pub total_value_locked: u64,
    /// Unique users in last 24h (if tracking state)
    #[prost(uint64, tag="6")]
    pub daily_active_users: u64,
    /// Total number of transactions
    #[prost(uint64, tag="7")]
    pub total_transactions: u64,
    /// Cross-chain analytics
    ///
    /// Metrics for each source-destination chain pair
    #[prost(message, repeated, tag="8")]
    pub chain_pairs: ::prost::alloc::vec::Vec<ChainPair>,
    /// Metrics for top tokens by volume
    #[prost(message, repeated, tag="9")]
    pub top_tokens: ::prost::alloc::vec::Vec<TokenMetrics>,
}
/// Metrics for a specific source-destination chain pair
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainPair {
    /// Source chain identifier
    #[prost(string, tag="1")]
    pub source_chain: ::prost::alloc::string::String,
    /// Destination chain identifier
    #[prost(string, tag="2")]
    pub destination_chain: ::prost::alloc::string::String,
    /// Number of messages between chains
    #[prost(uint64, tag="3")]
    pub message_count: u64,
    /// Number of token transfers
    #[prost(uint64, tag="4")]
    pub token_transfer_count: u64,
    /// Number of NFT transfers
    #[prost(uint64, tag="5")]
    pub nft_transfer_count: u64,
    /// Total volume in USD (if calculable)
    #[prost(double, tag="6")]
    pub token_volume_usd: f64,
}
/// Metrics for a specific token
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenMetrics {
    /// Token address
    #[prost(string, tag="1")]
    pub token_address: ::prost::alloc::string::String,
    /// Token symbol
    #[prost(string, tag="2")]
    pub token_symbol: ::prost::alloc::string::String,
    /// Token name
    #[prost(string, tag="3")]
    pub token_name: ::prost::alloc::string::String,
    /// Number of transfers
    #[prost(uint64, tag="4")]
    pub transfer_count: u64,
    /// Total volume in USD (if calculable)
    #[prost(double, tag="5")]
    pub volume_usd: f64,
    /// Unique users
    #[prost(uint64, tag="6")]
    pub unique_users: u64,
}
// @@protoc_insertion_point(module)
