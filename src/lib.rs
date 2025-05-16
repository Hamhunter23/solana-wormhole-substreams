// Imports from the auto-generated IDL bindings (from proto/program.proto)
// Check src/pb/mod.rs and src/pb/<some_path>/program.proto.rs to confirm these paths.
// Based on the template, idl::program::events seems correct for discriminators and structs.
// The path to `program.proto`'s bindings in the `pb` module might be different depending on protogen output.
// If errors persist for `pb::substreams::v1::program`, inspect `src/pb/mod.rs`.
mod idl; // Contains Anchor deserialization logic and discriminators (using the IDL)
mod pb;  // Contains Rust bindings for both auto-generated protos (program.proto) and our custom protos (wormhole/output.proto)
mod utils; // Contains utilities for chain mapping and token metadata

// Core Anchor/Solana/Substreams Imports
use anchor_lang::AnchorDeserialize; // For deserializing event data
use anchor_lang::Discriminator; // For accessing the DISCRIMINATOR constant
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD; // Use the standard base64 engine
use base64::Engine; // Trait needed for .decode() method
use substreams::errors::Error; // Standard Substreams error type for Result
use substreams_solana::pb::sf::solana::r#type::v1::Block; // The input block type
use std::collections::HashMap;

// Import OUR custom output protobufs 
use pb::wormhole::output::v1::{MessagePublication, MessagePublications};
use pb::wormhole::token_bridge::v1::{TokenTransfer, TokenTransfers};
use pb::wormhole::nft_bridge::v1::{NftTransfer as NFTTransfer, NftTransfers as NFTTransfers};
use pb::wormhole::combined::v1::{WormholeActivity, ChainPair, TokenMetrics};

// Import our utility functions
use utils::{chain_id_to_name, format_address_for_chain, get_token_metadata, extract_sequence_from_logs, extract_sender_address};

// Utility for base58 encoding transaction IDs and account addresses
use bs58;

// Optional: For logging within the substreams (uncomment if you want to use log::info! etc.)
// use substreams::log;

// Constants
// Allow dead_code for program IDs as they're used for documentation purposes
#[allow(dead_code)]
const CORE_BRIDGE_PROGRAM_ID: &str = "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth";
#[allow(dead_code)]
const TOKEN_BRIDGE_PROGRAM_ID: &str = "wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb";
#[allow(dead_code)]
const NFT_BRIDGE_PROGRAM_ID: &str = "WnFt12ZrnzZrFZkt2xsNsaNWoQribnuQ5B5FrDbwDhD";
#[allow(dead_code)]
const WORMHOLE_PROGRAM_ID: &str = "HDwcJBJXjL9FpJ7UBsYBtaDjsBUhuLCUYoz3zr8SWWaQ";

// The 8-byte Anchor discriminator for the MessagePublication event
// This comes from the auto-generated IDL bindings.
const MESSAGE_PUBLICATION_DISCRIMINATOR: &[u8] = idl::program::events::MessagePublication::DISCRIMINATOR;

// Token Bridge events discriminator constants
const TRANSFER_OUT_DISCRIMINATOR: &[u8] = idl::token_bridge::events::TransferOut::DISCRIMINATOR;
const TRANSFER_IN_DISCRIMINATOR: &[u8] = idl::token_bridge::events::TransferIn::DISCRIMINATOR;
const TRANSFER_NATIVE_DISCRIMINATOR: &[u8] = idl::token_bridge::events::TransferNative::DISCRIMINATOR;

// NFT Bridge events discriminator constants
const NFT_TRANSFER_DISCRIMINATOR: &[u8] = idl::nft_bridge::events::NFTTransfer::DISCRIMINATOR;
const NFT_RECEIVE_DISCRIMINATOR: &[u8] = idl::nft_bridge::events::NFTReceive::DISCRIMINATOR;

// Custom logger that only logs when there's activity
fn log_if_has_activity(message: &str, has_activity: bool) {
    if has_activity {
        substreams::log::info!("{}", message);
    }
}

// Implementation of the missing instruction_utils::get_instruction_logs function
fn get_instruction_logs<'a>(log_messages: &'a [String]) -> std::collections::HashMap<u32, Vec<&'a String>> {
    let mut result = std::collections::HashMap::new();
    let mut current_idx: Option<u32> = None;
    
    for message in log_messages {
        // Look for Solana's standard "Program log: Instruction: X" pattern
        if message.starts_with("Program log: Instruction: ") {
            if let Some(idx_str) = message.strip_prefix("Program log: Instruction: ") {
                if let Ok(idx) = idx_str.parse::<u32>() {
                    current_idx = Some(idx);
                    result.entry(idx).or_insert_with(Vec::new);
                }
            }
        } else if let Some(idx) = current_idx {
            // Associate this log message with the current instruction index
            if let Some(logs) = result.get_mut(&idx) {
                logs.push(message);
            }
        }
    }
    
    result
}

/// Prints block details in a format similar to Solana Explorer
fn print_block_details(blk: &Block) {
    let block_slot = blk.slot;
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);
    let block_hash = bs58::encode(&blk.blockhash).into_string();
    
    // Extract block height/epoch - it's an Option<BlockHeight>
    let epoch = match &blk.block_height {
        Some(height) => height.block_height.to_string(),
        None => "Unknown".to_string(),
    };
    
    // Rewards are directly in the block, not wrapped in an Option
    let rewards = &blk.rewards;
    
    let leader = if !rewards.is_empty() {
        if let Some(leader_reward) = rewards.iter().find(|r| r.reward_type == 1) { // 1 is for leader rewards
            bs58::encode(&leader_reward.pubkey).into_string()
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };
    
    let reward = if !rewards.is_empty() {
        let total_rewards: i64 = rewards.iter()
            .filter(|r| r.reward_type == 0) // 0 is for staking rewards
            .map(|r| r.lamports)
            .sum();
        format!("{:.6} SOL", total_rewards as f64 / 1_000_000_000.0)
    } else {
        "Unknown".to_string()
    };
    
    let mev_reward = if !rewards.is_empty() {
        let mev_rewards: i64 = rewards.iter()
            .filter(|r| r.reward_type == 2) // 2 might be for MEV rewards (assumption)
            .map(|r| r.lamports)
            .sum();
        format!("{:.6} SOL", mev_rewards as f64 / 1_000_000_000.0)
    } else {
        "Unknown".to_string()
    };
    
    let tx_count = blk.transactions().count();
    let previous_block_hash = bs58::encode(&blk.previous_blockhash).into_string();
    
    // Format timestamp as a human-readable date
    let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(block_timestamp, 0)
        .map(|dt| dt.format("%b %d, %Y %H:%M:%S %Z").to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    
    // Only print block details if there's activity
    substreams::log::info!("========== BLOCK DETAILS ==========");
    substreams::log::info!("Block: {}", block_slot);
    substreams::log::info!("Timestamp: {} ({})", datetime, block_timestamp);
    substreams::log::info!("Block Hash: {}", block_hash);
    substreams::log::info!("Epoch: {}", epoch);
    substreams::log::info!("Leader: {}", leader);
    substreams::log::info!("Reward: {}", reward);
    substreams::log::info!("MEV Reward: {}", mev_reward);
    substreams::log::info!("Transactions: Total {}", tx_count);
    substreams::log::info!("Previous Block Hash: {}", previous_block_hash);
    substreams::log::info!("==================================");
}

// Core Bridge module function
// Maps Wormhole Core Bridge message publications
#[substreams::handlers::map]
fn map_core_bridge_data(blk: Block) -> Result<MessagePublications, Error> {
    // Initialize the vector to collect MessagePublication events
    let mut publications: Vec<MessagePublication> = Vec::new();

    // Get block context data once per block
    let block_slot = blk.slot;
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);
    
    // Count transactions in the block
    let tx_count = blk.transactions().count();
    let mut found_activity = false;

    // Iterate through all transactions in the block
    // Filter out failed transactions early and get a reference to the transaction and its meta
    for (tx_index, (transaction, meta)) in blk.transactions().filter_map(|tx| {
        // Get an Option<&TransactionStatusMeta>
        tx.meta.as_ref()
          // Filter: Keep only if the transaction error field is None
          .filter(|meta| meta.err.is_none())
          // Map: If meta exists and has no error, return a tuple of (&ConfirmedTransaction, &TransactionStatusMeta)
          .map(|meta| (tx, meta))
    }).enumerate() {
        // Now 'transaction' is &ConfirmedTransaction and 'meta' is &TransactionStatusMeta

        // Get the transaction ID (signature) in Base58 format
        let tx_id = bs58::encode(&transaction.id()).into_string();
        
        // Check if this transaction involves the Core Bridge program
        let involves_core_bridge = if let Some(tx_msg) = transaction.transaction.as_ref().and_then(|tx| tx.message.as_ref()) {
            tx_msg.account_keys.iter().any(|key| {
                bs58::encode(key).into_string() == CORE_BRIDGE_PROGRAM_ID
            })
        } else {
            false
        };
        
        if involves_core_bridge {
            // If this is the first activity found, print block details
            if !found_activity {
                print_block_details(&blk);
                substreams::log::info!("Processing Core Bridge data for block {}", block_slot);
                substreams::log::info!("Block {} has {} transactions", block_slot, tx_count);
                found_activity = true;
            }
        
            substreams::log::info!("Transaction {} involves Core Bridge program: {}", tx_index + 1, tx_id);
            
            // Extract information from logs
            let mut sequence: u64 = 0;
            let mut emitter_account = String::new();
            let mut nonce: u32 = 0;
            let mut consistency_level: u8 = 0;
            
            // Look for specific log patterns
            for log in &meta.log_messages {
                // Extract sequence
                if log.contains("sequence:") || log.contains("Sequence:") {
                    if let Some(seq_part) = log.split("sequence:").nth(1).or_else(|| log.split("Sequence:").nth(1)) {
                        if let Some(seq_str) = seq_part.trim().split_whitespace().next() {
                            if let Ok(seq) = seq_str.parse::<u64>() {
                                sequence = seq;
                                substreams::log::info!("Found sequence: {}", sequence);
                            }
                        }
                    }
                }
                
                // Extract emitter
                if log.contains("emitter:") || log.contains("Emitter:") {
                    if let Some(emitter_part) = log.split("emitter:").nth(1).or_else(|| log.split("Emitter:").nth(1)) {
                        if let Some(emitter_str) = emitter_part.trim().split_whitespace().next() {
                            emitter_account = emitter_str.to_string();
                            substreams::log::info!("Found emitter: {}", emitter_account);
                        }
                    }
                }
                
                // Extract nonce
                if log.contains("nonce:") || log.contains("Nonce:") {
                    if let Some(nonce_part) = log.split("nonce:").nth(1).or_else(|| log.split("Nonce:").nth(1)) {
                        if let Some(nonce_str) = nonce_part.trim().split_whitespace().next() {
                            if let Ok(n) = nonce_str.parse::<u32>() {
                                nonce = n;
                                substreams::log::info!("Found nonce: {}", nonce);
                            }
                        }
                    }
                }
                
                // Extract consistency level
                if log.contains("consistency level:") || log.contains("Consistency Level:") {
                    if let Some(cl_part) = log.split("consistency level:").nth(1).or_else(|| log.split("Consistency Level:").nth(1)) {
                        if let Some(cl_str) = cl_part.trim().split_whitespace().next() {
                            if let Ok(cl) = cl_str.parse::<u8>() {
                                consistency_level = cl;
                                substreams::log::info!("Found consistency level: {}", consistency_level);
                            }
                        }
                    }
                }
            }
            
            // If we found a sequence, create a MessagePublication
            if sequence > 0 {
                let publication = MessagePublication {
                    tx_id: tx_id.clone(),
                    block_slot,
                    block_timestamp,
                    instruction_index: 0, // We don't know the instruction index from logs
                    inner_instruction_index: 0,
                    nonce,
                    payload: Vec::new(), // We can't extract payload from logs
                    emitter_account,
                    sequence,
                    consistency_level: consistency_level as u32,
                    event_timestamp: block_timestamp as u64,
                };
                
                publications.push(publication);
                substreams::log::info!("Added MessagePublication with sequence {}", sequence);
            }
        }
    }

    // Return the collected MessagePublication events wrapped in the MessagePublications container
    if found_activity {
        substreams::log::info!("Found {} message publications in block {}", publications.len(), block_slot);
    }
    
    // Return empty MessagePublications with no log if no activity was found
    Ok(MessagePublications { publications })
}

/// Extract token transfer data from a TransferOut event
fn parse_transfer_out(
    tx_id: &str, 
    block_slot: u64, 
    block_timestamp: i64, 
    inst_idx: u32,
    event: &idl::token_bridge::events::TransferOut,
    log_messages: &[&String],
    transaction: &substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction
) -> TokenTransfer {
    // Convert token address to Pubkey then to base58 string
    let token_address_bytes = event.token_address;
    let token_address = bs58::encode(&token_address_bytes).into_string();
    
    // Format source and destination chains
    let token_chain = chain_id_to_name(event.token_chain);
    let to_chain = chain_id_to_name(event.recipient_chain);
    
    // Format addresses
    let to_address = format_address_for_chain(event.recipient_chain, &event.recipient_address);
    let from_address = extract_sender_address(transaction, inst_idx);
    
    // Get token metadata if available
    let token_metadata = get_token_metadata(&token_address);
    
    // Extract sequence number
    let sequence = extract_sequence_from_logs(log_messages);
    
    // Create the token transfer event
    TokenTransfer {
        tx_id: tx_id.to_string(),
        block_slot,
        block_timestamp,
        instruction_index: inst_idx,
        inner_instruction_index: 0,
        token_address,
        token_chain,
        to_address,
        to_chain,
        from_address,
        amount: event.amount,
        fee: event.fee,
        token_symbol: token_metadata.as_ref().map_or("Unknown".to_string(), |m| m.symbol.clone()),
        token_decimals: token_metadata.as_ref().map_or(0, |m| m.decimals as u32),
        token_name: token_metadata.as_ref().map_or("Unknown Token".to_string(), |m| m.name.clone()),
        sequence,
        payload: "".to_string(), // No additional payload for standard transfers
    }
}

/// Extract token transfer data from a TransferIn event
fn parse_transfer_in(
    tx_id: &str, 
    block_slot: u64, 
    block_timestamp: i64, 
    inst_idx: u32,
    event: &idl::token_bridge::events::TransferIn,
    log_messages: &[&String],
    transaction: &substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction
) -> TokenTransfer {
    // Convert token address to Pubkey then to base58 string
    let token_address_bytes = event.token_address;
    let token_address = bs58::encode(&token_address_bytes).into_string();
    
    // Format source and destination chains
    let token_chain = chain_id_to_name(event.token_chain);
    let from_chain = chain_id_to_name(event.sender_chain);
    
    // Format addresses
    let from_address = format_address_for_chain(event.sender_chain, &event.sender_address);
    let to_address = extract_sender_address(transaction, inst_idx);
    
    // Get token metadata if available
    let token_metadata = get_token_metadata(&token_address);
    
    // Extract sequence number
    let sequence = extract_sequence_from_logs(log_messages);
    
    // Create the token transfer event
    TokenTransfer {
        tx_id: tx_id.to_string(),
        block_slot,
        block_timestamp,
        instruction_index: inst_idx,
        inner_instruction_index: 0,
        token_address,
        token_chain: from_chain.clone(), // Source chain is where token originates
        to_address,
        to_chain: "Solana".to_string(), // Destination is always Solana for TransferIn
        from_address,
        amount: event.amount,
        fee: 0, // TransferIn doesn't have a fee field
        token_symbol: token_metadata.as_ref().map_or("Unknown".to_string(), |m| m.symbol.clone()),
        token_decimals: token_metadata.as_ref().map_or(0, |m| m.decimals as u32),
        token_name: token_metadata.as_ref().map_or("Unknown Token".to_string(), |m| m.name.clone()),
        sequence,
        payload: "".to_string(), // No additional payload for standard transfers
    }
}

/// Extract token transfer data from a TransferNative event (SOL transfers)
fn parse_transfer_native(
    tx_id: &str, 
    block_slot: u64, 
    block_timestamp: i64, 
    inst_idx: u32,
    event: &idl::token_bridge::events::TransferNative,
    log_messages: &[&String],
    transaction: &substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction
) -> TokenTransfer {
    // Format destination chain
    let to_chain = chain_id_to_name(event.recipient_chain);
    
    // Format addresses
    let to_address = format_address_for_chain(event.recipient_chain, &event.recipient_address);
    let from_address = extract_sender_address(transaction, inst_idx);
    
    // Use SOL token metadata
    let token_metadata = get_token_metadata("So11111111111111111111111111111111111111112");
    
    // Extract sequence number
    let sequence = extract_sequence_from_logs(log_messages);
    
    // Create the token transfer event
    TokenTransfer {
        tx_id: tx_id.to_string(),
        block_slot,
        block_timestamp,
        instruction_index: inst_idx,
        inner_instruction_index: 0,
        token_address: "So11111111111111111111111111111111111111112".to_string(), // Native SOL
        token_chain: "Solana".to_string(),
        to_address,
        to_chain,
        from_address,
        amount: event.amount,
        fee: event.fee,
        token_symbol: token_metadata.as_ref().map_or("SOL".to_string(), |m| m.symbol.clone()),
        token_decimals: token_metadata.as_ref().map_or(9, |m| m.decimals as u32),
        token_name: token_metadata.as_ref().map_or("Solana".to_string(), |m| m.name.clone()),
        sequence,
        payload: "".to_string(), // No additional payload for standard transfers
    }
}

// Token Bridge module function
// Maps Wormhole Token Bridge token transfers
#[substreams::handlers::map]
fn map_token_bridge_data(blk: Block) -> Result<TokenTransfers, Error> {
    // Initialize the vector to collect TokenTransfer events
    let mut transfers: Vec<TokenTransfer> = Vec::new();

    // Get block context data once per block
    let block_slot = blk.slot;
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);
    
    // Count transactions in the block
    let tx_count = blk.transactions().count();
    let mut found_activity = false;

    // Iterate through all transactions in the block
    for (tx_index, (transaction, meta)) in blk.transactions().filter_map(|tx| {
        tx.meta.as_ref()
          .filter(|meta| meta.err.is_none())
          .map(|meta| (tx, meta))
    }).enumerate() {
        // Get the transaction ID (signature) in Base58 format
        let tx_id = bs58::encode(&transaction.id()).into_string();
        
        // Check if this transaction involves the Token Bridge program
        let involves_token_bridge = if let Some(tx_msg) = transaction.transaction.as_ref().and_then(|tx| tx.message.as_ref()) {
            tx_msg.account_keys.iter().any(|key| {
                bs58::encode(key).into_string() == TOKEN_BRIDGE_PROGRAM_ID
            })
        } else {
            false
        };
        
        if involves_token_bridge {
            // If this is the first activity found, print block details
            if !found_activity {
                print_block_details(&blk);
                substreams::log::info!("Processing Token Bridge data for block {}", block_slot);
                substreams::log::info!("Block {} has {} transactions", block_slot, tx_count);
                found_activity = true;
            }
            
            substreams::log::info!("Transaction {} involves Token Bridge program: {}", tx_index + 1, tx_id);
            
            // Extract information from logs
            let mut sequence: u64 = 0;
            let mut amount: u64 = 0;
            let mut fee: u64 = 0;
            let mut token_address = String::new();
            let mut token_chain = String::new();
            let mut to_chain = String::new();
            let mut to_address = String::new();
            let mut from_address = String::new();
            
            // Look for specific log patterns
            for log in &meta.log_messages {
                // Extract sequence
                if log.contains("sequence:") || log.contains("Sequence:") {
                    if let Some(seq_part) = log.split("sequence:").nth(1).or_else(|| log.split("Sequence:").nth(1)) {
                        if let Some(seq_str) = seq_part.trim().split_whitespace().next() {
                            if let Ok(seq) = seq_str.parse::<u64>() {
                                sequence = seq;
                                substreams::log::info!("Found sequence: {}", sequence);
                            }
                        }
                    }
                }
                
                // Extract amount
                if log.contains("amount:") || log.contains("Amount:") {
                    if let Some(amount_part) = log.split("amount:").nth(1).or_else(|| log.split("Amount:").nth(1)) {
                        if let Some(amount_str) = amount_part.trim().split_whitespace().next() {
                            if let Ok(amt) = amount_str.parse::<u64>() {
                                amount = amt;
                                substreams::log::info!("Found amount: {}", amount);
                            }
                        }
                    }
                }
                
                // Extract fee
                if log.contains("fee:") || log.contains("Fee:") {
                    if let Some(fee_part) = log.split("fee:").nth(1).or_else(|| log.split("Fee:").nth(1)) {
                        if let Some(fee_str) = fee_part.trim().split_whitespace().next() {
                            if let Ok(f) = fee_str.parse::<u64>() {
                                fee = f;
                                substreams::log::info!("Found fee: {}", fee);
                            }
                        }
                    }
                }
                
                // Extract token address
                if log.contains("token address:") || log.contains("Token address:") {
                    if let Some(addr_part) = log.split("token address:").nth(1).or_else(|| log.split("Token address:").nth(1)) {
                        if let Some(addr_str) = addr_part.trim().split_whitespace().next() {
                            token_address = addr_str.to_string();
                            substreams::log::info!("Found token address: {}", token_address);
                        }
                    }
                }
                
                // Extract token chain
                if log.contains("token chain:") || log.contains("Token chain:") {
                    if let Some(chain_part) = log.split("token chain:").nth(1).or_else(|| log.split("Token chain:").nth(1)) {
                        if let Some(chain_str) = chain_part.trim().split_whitespace().next() {
                            if let Ok(chain_id) = chain_str.parse::<u16>() {
                                token_chain = chain_id_to_name(chain_id);
                                substreams::log::info!("Found token chain: {}", token_chain);
                            }
                        }
                    }
                }
                
                // Extract destination chain
                if log.contains("recipient chain:") || log.contains("Recipient chain:") || log.contains("to chain:") {
                    if let Some(chain_part) = log.split("recipient chain:").nth(1)
                        .or_else(|| log.split("Recipient chain:").nth(1))
                        .or_else(|| log.split("to chain:").nth(1)) {
                        if let Some(chain_str) = chain_part.trim().split_whitespace().next() {
                            if let Ok(chain_id) = chain_str.parse::<u16>() {
                                to_chain = chain_id_to_name(chain_id);
                                substreams::log::info!("Found destination chain: {}", to_chain);
                            }
                        }
                    }
                }
                
                // Extract recipient address
                if log.contains("recipient:") || log.contains("Recipient:") {
                    if let Some(addr_part) = log.split("recipient:").nth(1).or_else(|| log.split("Recipient:").nth(1)) {
                        if let Some(addr_str) = addr_part.trim().split_whitespace().next() {
                            to_address = addr_str.to_string();
                            substreams::log::info!("Found recipient address: {}", to_address);
                        }
                    }
                }
            }
            
            // Extract sender address from transaction
            if from_address.is_empty() {
                from_address = extract_sender_address(transaction, 0);
            }
            
            // If token_chain is empty, default to Solana
            if token_chain.is_empty() {
                token_chain = "Solana".to_string();
            }
            
            // If to_chain is empty, default to destination being Solana (for incoming transfers)
            let is_incoming = to_chain.is_empty();
            if is_incoming {
                to_chain = "Solana".to_string();
            }
            
            // Get token metadata if available
            let token_metadata = get_token_metadata(&token_address);
            
            // If we found a sequence, create a TokenTransfer
            if sequence > 0 {
                let transfer = TokenTransfer {
                    tx_id: tx_id.clone(),
                    block_slot,
                    block_timestamp,
                    instruction_index: 0, // We don't know the instruction index from logs
                    inner_instruction_index: 0,
                    token_address,
                    token_chain,
                    to_address,
                    to_chain,
                    from_address,
                    amount,
                    fee,
                    token_symbol: token_metadata.as_ref().map_or("Unknown".to_string(), |m| m.symbol.clone()),
                    token_decimals: token_metadata.as_ref().map_or(0, |m| m.decimals as u32),
                    token_name: token_metadata.as_ref().map_or("Unknown Token".to_string(), |m| m.name.clone()),
                    sequence,
                    payload: "".to_string(),
                };
                
                transfers.push(transfer);
                substreams::log::info!("Added TokenTransfer with sequence {}", sequence);
            }
        }
    }
    
    if found_activity {
        substreams::log::info!("Found {} token transfers in block {}", transfers.len(), block_slot);
    }
    Ok(TokenTransfers { transfers })
}

// NFT Bridge module function
// Maps Wormhole NFT Bridge transfers
#[substreams::handlers::map]
fn map_nft_bridge_data(blk: Block) -> Result<NFTTransfers, Error> {
    // Initialize the vector to collect NFTTransfer events
    let mut transfers: Vec<NFTTransfer> = Vec::new();

    // Get block context data once per block
    let block_slot = blk.slot;
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);

    // Count transactions in the block
    let tx_count = blk.transactions().count();
    let mut found_activity = false;

    // Iterate through all transactions in the block
    for (tx_index, (transaction, meta)) in blk.transactions().filter_map(|tx| {
        tx.meta.as_ref()
          .filter(|meta| meta.err.is_none())
          .map(|meta| (tx, meta))
    }).enumerate() {
        // Get the transaction ID (signature) in Base58 format
        let tx_id = bs58::encode(&transaction.id()).into_string();
        
        // Check if this transaction involves the NFT Bridge program
        let involves_nft_bridge = if let Some(tx_msg) = transaction.transaction.as_ref().and_then(|tx| tx.message.as_ref()) {
            tx_msg.account_keys.iter().any(|key| {
                bs58::encode(key).into_string() == NFT_BRIDGE_PROGRAM_ID
            })
        } else {
            false
        };
        
        if involves_nft_bridge {
            // If this is the first activity found, print block details
            if !found_activity {
                print_block_details(&blk);
                substreams::log::info!("Processing NFT Bridge data for block {}", block_slot);
                substreams::log::info!("Block {} has {} transactions", block_slot, tx_count);
                found_activity = true;
            }
            
            substreams::log::info!("Transaction {} involves NFT Bridge program: {}", tx_index + 1, tx_id);
            
            // Extract information from logs
            let mut sequence: u64 = 0;
            let mut token_id: u64 = 0;
            let mut nft_address = String::new();
            let mut to_chain = String::new();
            let mut to_address = String::new();
            let mut from_address = String::new();
            let mut uri = String::new();
            let mut name = String::new();
            let mut symbol = String::new();
            
            // Look for specific log patterns
            for log in &meta.log_messages {
                // Extract sequence
                if log.contains("sequence:") || log.contains("Sequence:") {
                    if let Some(seq_part) = log.split("sequence:").nth(1).or_else(|| log.split("Sequence:").nth(1)) {
                        if let Some(seq_str) = seq_part.trim().split_whitespace().next() {
                            if let Ok(seq) = seq_str.parse::<u64>() {
                                sequence = seq;
                                substreams::log::info!("Found sequence: {}", sequence);
                            }
                        }
                    }
                }
                
                // Extract token ID
                if log.contains("token ID:") || log.contains("Token ID:") || log.contains("tokenId:") {
                    if let Some(id_part) = log.split("token ID:").nth(1)
                        .or_else(|| log.split("Token ID:").nth(1))
                        .or_else(|| log.split("tokenId:").nth(1)) {
                        if let Some(id_str) = id_part.trim().split_whitespace().next() {
                            if let Ok(id) = id_str.parse::<u64>() {
                                token_id = id;
                                substreams::log::info!("Found token ID: {}", token_id);
                            }
                        }
                    }
                }
                
                // Extract NFT address
                if log.contains("token address:") || log.contains("Token address:") || log.contains("NFT:") {
                    if let Some(addr_part) = log.split("token address:").nth(1)
                        .or_else(|| log.split("Token address:").nth(1))
                        .or_else(|| log.split("NFT:").nth(1)) {
                        if let Some(addr_str) = addr_part.trim().split_whitespace().next() {
                            nft_address = addr_str.to_string();
                            substreams::log::info!("Found NFT address: {}", nft_address);
                        }
                    }
                }
                
                // Extract destination chain
                if log.contains("recipient chain:") || log.contains("Recipient chain:") || log.contains("to chain:") {
                    if let Some(chain_part) = log.split("recipient chain:").nth(1)
                        .or_else(|| log.split("Recipient chain:").nth(1))
                        .or_else(|| log.split("to chain:").nth(1)) {
                        if let Some(chain_str) = chain_part.trim().split_whitespace().next() {
                            if let Ok(chain_id) = chain_str.parse::<u16>() {
                                to_chain = chain_id_to_name(chain_id);
                                substreams::log::info!("Found destination chain: {}", to_chain);
                            }
                        }
                    }
                }
                
                // Extract recipient address
                if log.contains("recipient:") || log.contains("Recipient:") {
                    if let Some(addr_part) = log.split("recipient:").nth(1).or_else(|| log.split("Recipient:").nth(1)) {
                        if let Some(addr_str) = addr_part.trim().split_whitespace().next() {
                            to_address = addr_str.to_string();
                            substreams::log::info!("Found recipient address: {}", to_address);
                        }
                    }
                }
                
                // Extract URI
                if log.contains("URI:") || log.contains("uri:") {
                    if let Some(uri_part) = log.split("URI:").nth(1).or_else(|| log.split("uri:").nth(1)) {
                        uri = uri_part.trim().to_string();
                        substreams::log::info!("Found URI: {}", uri);
                    }
                }
                
                // Extract name
                if log.contains("name:") || log.contains("Name:") {
                    if let Some(name_part) = log.split("name:").nth(1).or_else(|| log.split("Name:").nth(1)) {
                        name = name_part.trim().to_string();
                        substreams::log::info!("Found name: {}", name);
                    }
                }
                
                // Extract symbol
                if log.contains("symbol:") || log.contains("Symbol:") {
                    if let Some(symbol_part) = log.split("symbol:").nth(1).or_else(|| log.split("Symbol:").nth(1)) {
                        symbol = symbol_part.trim().to_string();
                        substreams::log::info!("Found symbol: {}", symbol);
                    }
                }
            }
            
            // Extract sender address from transaction
            if from_address.is_empty() {
                from_address = extract_sender_address(transaction, 0);
            }
            
            // If we found a sequence, create an NFTTransfer
            if sequence > 0 {
                let transfer = NFTTransfer {
                    tx_id: tx_id.clone(),
                    block_slot,
                    block_timestamp,
                    instruction_index: 0, // We don't know the instruction index from logs
                    inner_instruction_index: 0,
                    nft_address,
                    nft_chain: "Solana".to_string(), // Source chain is Solana
                    to_address,
                    to_chain,
                    from_address,
                    token_id,
                    uri,
                    name,
                    symbol,
                    sequence,
                    payload: "".to_string(),
                };
                
                transfers.push(transfer);
                substreams::log::info!("Added NFTTransfer with sequence {}", sequence);
            }
        }
    }
    
    if found_activity {
        substreams::log::info!("Found {} NFT transfers in block {}", transfers.len(), block_slot);
    }
    Ok(NFTTransfers { transfers })
}

// Wormhole Program module function 
// This is similar to the Core Bridge but for the main Wormhole program
#[substreams::handlers::map]
fn map_wormhole_program_data(blk: Block) -> Result<MessagePublications, Error> {
    // Initialize the vector to collect MessagePublication events
    let mut publications: Vec<MessagePublication> = Vec::new();

    // Get block context data once per block
    let block_slot = blk.slot;
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);
    
    let mut found_activity = false;
    let tx_count = blk.transactions().count();

    // Iterate through all transactions in the block
    for (tx_index, (transaction, meta)) in blk.transactions().filter_map(|tx| {
        tx.meta.as_ref()
          .filter(|meta| meta.err.is_none())
          .map(|meta| (tx, meta))
    }).enumerate() {
        let tx_id = bs58::encode(&transaction.id()).into_string();
        
        // Check if this transaction involves the Wormhole program
        let involves_wormhole = if let Some(tx_msg) = transaction.transaction.as_ref().and_then(|tx| tx.message.as_ref()) {
            tx_msg.account_keys.iter().any(|key| {
                bs58::encode(key).into_string() == WORMHOLE_PROGRAM_ID
            })
        } else {
            false
        };
        
        if involves_wormhole {
            // If this is the first activity found, print block details
            if !found_activity {
                print_block_details(&blk);
                substreams::log::info!("Processing Wormhole Program data for block {}", block_slot);
                substreams::log::info!("Block {} has {} transactions", block_slot, tx_count);
                found_activity = true;
            }
            
            substreams::log::info!("Transaction {} involves Wormhole Program: {}", tx_index + 1, tx_id);
            
            let program_log_messages = get_instruction_logs(&meta.log_messages);

            // Process each instruction's logs - similar to Core Bridge
            for (inst_idx, log_messages) in program_log_messages.iter() {
                for log_message in log_messages.iter() {
                    if log_message.starts_with("Program data: ") {
                        let base64_data = &log_message["Program data: ".len()..];
                        
                        if let Ok(decoded_data) = BASE64_STANDARD.decode(base64_data) {
                            if decoded_data.len() >= 8 {
                                if &decoded_data[0..8] == MESSAGE_PUBLICATION_DISCRIMINATOR {
                                    substreams::log::info!("Found Message Publication event in tx: {}", tx_id);
                                    
                                    let mut slice_u8: &[u8] = &decoded_data[8..];
                                    if let Ok(event) = idl::program::events::MessagePublication::deserialize(&mut slice_u8) {
                                        let publication = MessagePublication {
                                            tx_id: tx_id.clone(),
                                            block_slot,
                                            block_timestamp,
                                            instruction_index: *inst_idx,
                                            inner_instruction_index: 0,
                                            nonce: event.nonce,
                                            payload: event.payload,
                                            emitter_account: bs58::encode(&event.emitter_account.to_bytes()).into_string(),
                                            sequence: event.sequence,
                                            consistency_level: event.consistency_level as u32,
                                            event_timestamp: event.timestamp,
                                        };
                                        
                                        publications.push(publication);
                                        substreams::log::info!("Added MessagePublication with sequence {}", event.sequence);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    if found_activity {
        substreams::log::info!("Found {} message publications in block {}", publications.len(), block_slot);
    }
    Ok(MessagePublications { publications })
}

// Combined module function
// Aggregates data from all Wormhole bridges
#[substreams::handlers::map]
fn combine_wormhole_activity(
    core_bridge: MessagePublications,
    token_bridge: TokenTransfers,
    nft_bridge: NFTTransfers,
    wormhole_program: MessagePublications,
) -> Result<WormholeActivity, Error> {
    // Get all the messages from the various bridges
    let mut core_messages = Vec::new();
    core_messages.extend(core_bridge.publications);
    core_messages.extend(wormhole_program.publications);
    
    let token_transfers = token_bridge.transfers;
    let nft_transfers = nft_bridge.transfers;
    
    // Only log if we have activity
    let has_activity = !core_messages.is_empty() || !token_transfers.is_empty() || !nft_transfers.is_empty();
    
    // Calculate the timestamp from the latest message or transfer
    let mut latest_timestamp = 0;
    for msg in &core_messages {
        if msg.block_timestamp > latest_timestamp {
            latest_timestamp = msg.block_timestamp;
        }
    }
    
    for transfer in &token_transfers {
        if transfer.block_timestamp > latest_timestamp {
            latest_timestamp = transfer.block_timestamp;
        }
    }
    
    for transfer in &nft_transfers {
        if transfer.block_timestamp > latest_timestamp {
            latest_timestamp = transfer.block_timestamp;
        }
    }
    
    // Calculate total transactions
    let total_transactions = (core_messages.len() + token_transfers.len() + nft_transfers.len()) as u64;
    
    // Generate chain pair analytics
    let mut chain_pair_map: HashMap<(String, String), ChainPair> = HashMap::new();
    
    // Process token transfers for chain pairs
    for transfer in &token_transfers {
        let key = (transfer.token_chain.clone(), transfer.to_chain.clone());
        let entry = chain_pair_map.entry(key.clone()).or_insert_with(|| ChainPair {
            source_chain: key.0.clone(),
            destination_chain: key.1.clone(),
            message_count: 0,
            token_transfer_count: 0,
            nft_transfer_count: 0,
            token_volume_usd: 0.0,
        });
        
        entry.token_transfer_count += 1;
        
        // Calculate USD value if possible based on token type
        if let Some(token_metadata) = get_token_metadata(&transfer.token_address) {
            let decimal_factor = 10_f64.powi(token_metadata.decimals as i32);
            let token_amount = transfer.amount as f64 / decimal_factor;
            
            // This is where you would look up token price from an oracle
            // For now we'll use simplified price estimates for common tokens
            let usd_price = match token_metadata.symbol.as_str() {
                "USDC" | "USDT" => 1.0,
                "SOL" => 63.0, // Example price
                "ETH" | "WETH" => 2200.0, // Example price
                "BTC" | "WBTC" => 52000.0, // Example price
                _ => 0.0 // Unknown token price
            };
            
            let usd_value = token_amount * usd_price;
            entry.token_volume_usd += usd_value;
        }
    }
    
    // Process NFT transfers for chain pairs
    for transfer in &nft_transfers {
        let key = (transfer.nft_chain.clone(), transfer.to_chain.clone());
        let entry = chain_pair_map.entry(key.clone()).or_insert_with(|| ChainPair {
            source_chain: key.0.clone(),
            destination_chain: key.1.clone(),
            message_count: 0,
            token_transfer_count: 0,
            nft_transfer_count: 0,
            token_volume_usd: 0.0,
        });
        
        entry.nft_transfer_count += 1;
    }
    
    // Track message counts for chain pairs
    for msg in &core_messages {
        // In a real implementation, you would extract source and destination chains
        // from the message payload. For now, we'll use a simplified approach.
        // This is where more advanced parsing of the VAA would come in.
        
        // The message metadata typically doesn't contain the destination chain directly,
        // it would be in the parsed VAA data
    }
    
    // Convert the chain pair map to a vector
    let chain_pairs = chain_pair_map.into_values().collect();
    
    // Track token metrics by token address
    let mut token_metrics_map: HashMap<String, (u64, f64, u64, String, String)> = HashMap::new();
    
    // Process token transfers for token metrics
    for transfer in &token_transfers {
        let entry = token_metrics_map.entry(transfer.token_address.clone()).or_insert_with(|| {
            (0, 0.0, 0, transfer.token_symbol.clone(), transfer.token_name.clone())
        });
        
        // Update metrics
        entry.0 += 1; // Increment transfer count
        
        // Calculate USD value if possible
        if transfer.token_decimals > 0 {
            let decimal_factor = 10_f64.powi(transfer.token_decimals as i32);
            let token_amount = transfer.amount as f64 / decimal_factor;
            
            // Same price lookup as above
            let usd_price = match transfer.token_symbol.as_str() {
                "USDC" | "USDT" => 1.0,
                "SOL" => 63.0,
                "ETH" | "WETH" => 2200.0,
                "BTC" | "WBTC" => 52000.0,
                _ => 0.0
            };
            
            entry.1 += token_amount * usd_price; // Add USD volume
        }
        
        // Track unique users (simplified - would need a Set in real implementation)
        entry.2 += 1; // This is a simplification, real implementation would track unique addresses
    }
    
    // Convert token metrics map to a vector
    let top_tokens = token_metrics_map.into_iter().map(|(addr, (count, volume, users, symbol, name))| {
        TokenMetrics {
            token_address: addr,
            token_symbol: symbol,
            token_name: name,
            transfer_count: count,
            volume_usd: volume,
            unique_users: users,
        }
    }).collect();
    
    // Calculate TVL and DAU (simplified - real implementation would be more complex)
    let total_value_locked = calculate_tvl(&token_transfers);
    let daily_active_users = calculate_dau(&token_transfers, &nft_transfers, &core_messages);
    
    let wormhole_activity = WormholeActivity {
        core_messages,
        token_transfers,
        nft_transfers,
        timestamp: latest_timestamp,
        total_value_locked,
        daily_active_users,
        total_transactions,
        chain_pairs,
        top_tokens,
    };
    
    Ok(wormhole_activity)
}

// Calculate total value locked (simplified)
fn calculate_tvl(token_transfers: &[TokenTransfer]) -> u64 {
    let mut tvl = 0.0;
    
    for transfer in token_transfers {
        if let Some(token_metadata) = get_token_metadata(&transfer.token_address) {
            let decimal_factor = 10_f64.powi(token_metadata.decimals as i32);
            let token_amount = transfer.amount as f64 / decimal_factor;
            
            let usd_price = match token_metadata.symbol.as_str() {
                "USDC" | "USDT" => 1.0,
                "SOL" => 63.0,
                "ETH" | "WETH" => 2200.0,
                "BTC" | "WBTC" => 52000.0,
                _ => 0.0
            };
            
            tvl += token_amount * usd_price;
        }
    }
    
    tvl as u64
}

// Calculate daily active users (simplified)
fn calculate_dau(token_transfers: &[TokenTransfer], nft_transfers: &[NFTTransfer], core_messages: &[MessagePublication]) -> u64 {
    let mut unique_addresses = std::collections::HashSet::new();
    
    // Add token transfer participants
    for transfer in token_transfers {
        unique_addresses.insert(transfer.from_address.clone());
        unique_addresses.insert(transfer.to_address.clone());
    }
    
    // Add NFT transfer participants
    for transfer in nft_transfers {
        unique_addresses.insert(transfer.from_address.clone());
        unique_addresses.insert(transfer.to_address.clone());
    }
    
    // Add core message emitters
    for msg in core_messages {
        unique_addresses.insert(msg.emitter_account.clone());
    }
    
    unique_addresses.len() as u64
}