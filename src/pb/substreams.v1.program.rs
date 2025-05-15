// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub message_publication_event_list: ::prost::alloc::vec::Vec<MessagePublicationEvent>,
    #[prost(message, repeated, tag="2")]
    pub guardian_set_appended_event_list: ::prost::alloc::vec::Vec<GuardianSetAppendedEvent>,
    #[prost(message, repeated, tag="3")]
    pub initialize_instruction_list: ::prost::alloc::vec::Vec<InitializeInstruction>,
    #[prost(message, repeated, tag="4")]
    pub set_fees_instruction_list: ::prost::alloc::vec::Vec<SetFeesInstruction>,
    #[prost(message, repeated, tag="5")]
    pub transfer_fees_instruction_list: ::prost::alloc::vec::Vec<TransferFeesInstruction>,
    #[prost(message, repeated, tag="6")]
    pub set_governance_bot_instruction_list: ::prost::alloc::vec::Vec<SetGovernanceBotInstruction>,
    #[prost(message, repeated, tag="7")]
    pub post_message_instruction_list: ::prost::alloc::vec::Vec<PostMessageInstruction>,
    #[prost(message, repeated, tag="8")]
    pub postvaa_instruction_list: ::prost::alloc::vec::Vec<PostvaaInstruction>,
    #[prost(message, repeated, tag="9")]
    pub parse_and_postvaa_instruction_list: ::prost::alloc::vec::Vec<ParseAndPostvaaInstruction>,
    #[prost(message, repeated, tag="10")]
    pub upgrade_guardian_set_instruction_list: ::prost::alloc::vec::Vec<UpgradeGuardianSetInstruction>,
    #[prost(message, repeated, tag="11")]
    pub set_paused_instruction_list: ::prost::alloc::vec::Vec<SetPausedInstruction>,
    #[prost(message, repeated, tag="12")]
    pub post_message_fast_instruction_list: ::prost::alloc::vec::Vec<PostMessageFastInstruction>,
    #[prost(message, repeated, tag="13")]
    pub set_upgrade_buffer_instruction_list: ::prost::alloc::vec::Vec<SetUpgradeBufferInstruction>,
    #[prost(message, repeated, tag="14")]
    pub submit_vaa_fast_instruction_list: ::prost::alloc::vec::Vec<SubmitVaaFastInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePublicationEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub nonce: u32,
    #[prost(bytes="vec", tag="3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub emitter_account: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub sequence: u64,
    #[prost(uint64, tag="6")]
    pub consistency_level: u64,
    #[prost(uint64, tag="7")]
    pub timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuardianSetAppendedEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub guardian_set_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub governance_vaas_enabled: bool,
    #[prost(string, tag="3")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_config: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_fee_collector: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_rent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFeesInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub batch_price: u64,
    #[prost(string, tag="3")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_config: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferFeesInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(string, tag="3")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_config: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_fee_collector: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetGovernanceBotInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_config: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_governance_bot: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostMessageInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub nonce: u32,
    #[prost(bytes="vec", tag="3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub consistency_level: u64,
    #[prost(string, tag="5")]
    pub acct_emitter: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_message: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_config: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_fee_collector: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_sequence: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_rent: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_system_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostvaaInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub vaa: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_config: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_vaa: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_signature_set: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_rent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParseAndPostvaaInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub vaa: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_config: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_vaa: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_signature_set: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_rent: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_wormhole_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeGuardianSetInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub vaa: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_config: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_new_guardian_set: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_rent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPausedInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub paused: bool,
    #[prost(string, tag="3")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_config: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostMessageFastInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub nonce: u32,
    #[prost(bytes="vec", tag="3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub consistency_level: u64,
    #[prost(string, tag="5")]
    pub acct_emitter: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_message: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_config: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_sequence: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_rent: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_signature_set: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpgradeBufferInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub upgrade_buffer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_config: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitVaaFastInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub vaa: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_signature_set: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_rent: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
