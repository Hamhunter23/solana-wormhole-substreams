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
// These would be determined from the IDL or from inspection of real events
const NFT_TRANSFER_DISCRIMINATOR: &[u8] = &[233, 146, 209, 97, 112, 27, 49, 37]; // Example value

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

// Core Bridge module function
// Maps Wormhole Core Bridge message publications
#[substreams::handlers::map]
fn map_core_bridge_data(blk: Block) -> Result<MessagePublications, Error> {
    // Initialize the vector to collect MessagePublication events
    let mut publications: Vec<MessagePublication> = Vec::new();

    // Get block context data once per block
    let block_slot = blk.slot;
    // Get the block timestamp (unwrap_or(0) provides a default if timestamp is missing, though rare for blocks)
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);

    // Iterate through all transactions in the block
    // Filter out failed transactions early and get a reference to the transaction and its meta
    for (transaction, meta) in blk.transactions().filter_map(|tx| {
        // Get an Option<&TransactionStatusMeta>
        tx.meta.as_ref()
          // Filter: Keep only if the transaction error field is None
          .filter(|meta| meta.err.is_none())
          // Map: If meta exists and has no error, return a tuple of (&ConfirmedTransaction, &TransactionStatusMeta)
          .map(|meta| (tx, meta))
    }) {
        // Now 'transaction' is &ConfirmedTransaction and 'meta' is &TransactionStatusMeta

        // Get the transaction ID (signature) in Base58 format
        let tx_id = bs58::encode(&transaction.id()).into_string();

        // Use substreams-solana helper to group log messages by the instruction index they originated from
        // This helps correlate logs to the top-level instruction that triggered them
        let program_log_messages = get_instruction_logs(&meta.log_messages);

        // Iterate through the log messages, grouped by the instruction index
        // The key `inst_idx` is the index of the top-level instruction
        for (inst_idx, log_messages) in program_log_messages.iter() {
            // Iterate through each log message within this instruction group
            for log_message in log_messages.iter() {

                // Anchor event data is typically emitted in logs starting with "Program data: "
                // followed by base64 encoded data (discriminator + serialized event data)
                if log_message.starts_with("Program data: ") {
                    let base64_data = &log_message["Program data: ".len()..];

                    // Attempt to decode the base64 data using the standard engine
                    if let Ok(decoded_data) = BASE64_STANDARD.decode(base64_data) {
                         // Check if the decoded data is long enough (at least 8 bytes for discriminator)
                         if decoded_data.len() >= 8 {
                            // Check if the first 8 bytes match the MessagePublication discriminator
                            if &decoded_data[0..8] == MESSAGE_PUBLICATION_DISCRIMINATOR { // Compare slices directly

                                 // Attempt to deserialize the rest of the data (after the 8-byte discriminator)
                                 let mut slice_u8: &[u8] = &decoded_data[8..];
                                 // Assuming `idl::program::events::MessagePublication` is the auto-generated struct
                                 // from your IDL that implements AnchorDeserialize
                                 if let Ok(event) = idl::program::events::MessagePublication::deserialize(&mut slice_u8) {

                                     // Successfully decoded a MessagePublication event!
                                     // Create an instance of our custom output Protobuf struct
                                     let publication = MessagePublication {
                                        tx_id: tx_id.clone(), // Clone tx_id for each publication
                                        block_slot,
                                        block_timestamp,
                                        instruction_index: *inst_idx, // Dereference the instruction index (u32)
                                        inner_instruction_index: 0, // Simple log parsing might not distinguish inner index easily.
                                        nonce: event.nonce, // u32
                                        payload: event.payload, // bytes (Vec<u8>)
                                        emitter_account: bs58::encode(&event.emitter_account.to_bytes()).into_string(), // Convert Anchor Pubkey to bytes, then base58 string
                                        sequence: event.sequence, // u64
                                        consistency_level: event.consistency_level as u32, // Cast u8 to u32 for Protobuf
                                        event_timestamp: event.timestamp, // u64
                                     };

                                     // Add the populated event data to our collection
                                     publications.push(publication);
                                 }
                             }
                         }
                     }
                }
            }
        } // End nested for (inst_idx, log_messages) loop

    } // End outer for transaction loop

    // Return the collected MessagePublication events wrapped in the MessagePublications container
    Ok(MessagePublications { publications })
} // End of the map_core_bridge_data function

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

    // Iterate through all transactions in the block
    for (transaction, meta) in blk.transactions().filter_map(|tx| {
        tx.meta.as_ref()
          .filter(|meta| meta.err.is_none())
          .map(|meta| (tx, meta))
    }) {
        let tx_id = bs58::encode(&transaction.id()).into_string();
        let program_log_messages = get_instruction_logs(&meta.log_messages);

        // Process each instruction's logs
        for (inst_idx, log_messages) in program_log_messages.iter() {
            for log_message in log_messages.iter() {
                if log_message.starts_with("Program data: ") {
                    let base64_data = &log_message["Program data: ".len()..];
                    
                    if let Ok(decoded_data) = BASE64_STANDARD.decode(base64_data) {
                        if decoded_data.len() >= 8 {
                            // Check which event type we're dealing with
                            let discriminator = &decoded_data[0..8];
                            
                            if discriminator == TRANSFER_OUT_DISCRIMINATOR {
                                // Parse TransferOut event
                                let mut slice_u8: &[u8] = &decoded_data[8..];
                                if let Ok(event) = idl::token_bridge::events::TransferOut::deserialize(&mut slice_u8) {
                                    let transfer = parse_transfer_out(
                                        &tx_id, 
                                        block_slot, 
                                        block_timestamp, 
                                        *inst_idx,
                                        &event,
                                        log_messages,
                                        transaction
                                    );
                                    transfers.push(transfer);
                                }
                            } else if discriminator == TRANSFER_IN_DISCRIMINATOR {
                                // Parse TransferIn event
                                let mut slice_u8: &[u8] = &decoded_data[8..];
                                if let Ok(event) = idl::token_bridge::events::TransferIn::deserialize(&mut slice_u8) {
                                    let transfer = parse_transfer_in(
                                        &tx_id, 
                                        block_slot, 
                                        block_timestamp, 
                                        *inst_idx,
                                        &event,
                                        log_messages,
                                        transaction
                                    );
                                    transfers.push(transfer);
                                }
                            } else if discriminator == TRANSFER_NATIVE_DISCRIMINATOR {
                                // Parse TransferNative event
                                let mut slice_u8: &[u8] = &decoded_data[8..];
                                if let Ok(event) = idl::token_bridge::events::TransferNative::deserialize(&mut slice_u8) {
                                    let transfer = parse_transfer_native(
                                        &tx_id, 
                                        block_slot, 
                                        block_timestamp, 
                                        *inst_idx,
                                        &event,
                                        log_messages,
                                        transaction
                                    );
                                    transfers.push(transfer);
                                }
                            }
                        }
                    }
                }
            }
        }
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

    // Iterate through all transactions in the block
    for (transaction, meta) in blk.transactions().filter_map(|tx| {
        tx.meta.as_ref()
          .filter(|meta| meta.err.is_none())
          .map(|meta| (tx, meta))
    }) {
        let tx_id = bs58::encode(&transaction.id()).into_string();
        let program_log_messages = get_instruction_logs(&meta.log_messages);

        // Process each instruction's logs
        for (inst_idx, log_messages) in program_log_messages.iter() {
            for log_message in log_messages.iter() {
                if log_message.starts_with("Program data: ") {
                    let base64_data = &log_message["Program data: ".len()..];
                    
                    if let Ok(decoded_data) = BASE64_STANDARD.decode(base64_data) {
                        if decoded_data.len() >= 8 {
                            // Check NFT transfer discriminator
                            if &decoded_data[0..8] == NFT_TRANSFER_DISCRIMINATOR {
                                // In a real implementation, you would deserialize the NFT transfer data
                                // and extract all the NFT-specific fields.
                                // This is a simplified example:
                                
                                // Parse key data from the logs or transaction
                                // These would be extracted from the transaction or the decoded event
                                let nft_address = "sample_nft_address".to_string(); // Placeholder
                                let nft_chain = "1".to_string(); // Solana chain ID
                                let to_address = "recipient_address".to_string(); // Placeholder
                                let to_chain = "2".to_string(); // Placeholder destination chain
                                let from_address = "sender_address".to_string(); // Placeholder 
                                let token_id = 12345; // Placeholder
                                let sequence = 12345; // Placeholder
                                
                                let transfer = NFTTransfer {
                                    tx_id: tx_id.clone(),
                                    block_slot,
                                    block_timestamp,
                                    instruction_index: *inst_idx,
                                    inner_instruction_index: 0,
                                    nft_address,
                                    nft_chain,
                                    to_address,
                                    to_chain,
                                    from_address,
                                    token_id,
                                    uri: "https://example.com/nft/12345".to_string(), // Placeholder
                                    name: "Sample NFT".to_string(), // Placeholder
                                    symbol: "SNFT".to_string(), // Placeholder
                                    sequence,
                                    payload: "".to_string(), // Placeholder
                                };
                                
                                transfers.push(transfer);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(NFTTransfers { transfers })
}

// Wormhole Program module function 
// This is similar to the Core Bridge but for the main Wormhole program
#[substreams::handlers::map]
fn map_wormhole_program_data(blk: Block) -> Result<MessagePublications, Error> {
    // This is similar to map_core_bridge_data but for the Wormhole program
    // We'll implement it with the same logic as the Core Bridge
    
    // Initialize the vector to collect MessagePublication events
    let mut publications: Vec<MessagePublication> = Vec::new();

    // Get block context data once per block
    let block_slot = blk.slot;
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);

    // Iterate through all transactions in the block
    for (transaction, meta) in blk.transactions().filter_map(|tx| {
        tx.meta.as_ref()
          .filter(|meta| meta.err.is_none())
          .map(|meta| (tx, meta))
    }) {
        let tx_id = bs58::encode(&transaction.id()).into_string();
        let program_log_messages = get_instruction_logs(&meta.log_messages);

        // Process each instruction's logs - similar to Core Bridge
        for (inst_idx, log_messages) in program_log_messages.iter() {
            for log_message in log_messages.iter() {
                if log_message.starts_with("Program data: ") {
                    let base64_data = &log_message["Program data: ".len()..];
                    
                    if let Ok(decoded_data) = BASE64_STANDARD.decode(base64_data) {
                        if decoded_data.len() >= 8 {
                            if &decoded_data[0..8] == MESSAGE_PUBLICATION_DISCRIMINATOR {
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
                                }
                            }
                        }
                    }
                }
            }
        }
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