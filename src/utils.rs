use std::collections::HashMap;
use std::sync::OnceLock;
use anchor_lang::prelude::Pubkey;

// Cache for known token metadata
static TOKEN_METADATA_CACHE: OnceLock<HashMap<String, TokenMetadata>> = OnceLock::new();

/// Token metadata structure
#[derive(Clone, Debug)]
pub struct TokenMetadata {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

/// Convert chain ID to a human-readable name
pub fn chain_id_to_name(chain_id: u16) -> String {
    match chain_id {
        1 => "Solana".to_string(),
        2 => "Ethereum".to_string(),
        3 => "Terra".to_string(),
        4 => "BSC".to_string(),
        5 => "Polygon".to_string(),
        6 => "Avalanche".to_string(),
        7 => "Oasis".to_string(),
        8 => "Algorand".to_string(),
        9 => "Aurora".to_string(),
        10 => "Fantom".to_string(),
        11 => "Karura".to_string(),
        12 => "Acala".to_string(),
        13 => "Klaytn".to_string(),
        14 => "Celo".to_string(),
        15 => "NEAR".to_string(),
        16 => "Moonbeam".to_string(),
        17 => "Neon".to_string(),
        18 => "Terra2".to_string(),
        19 => "Injective".to_string(),
        20 => "Sui".to_string(),
        21 => "Aptos".to_string(),
        22 => "Arbitrum".to_string(),
        23 => "Optimism".to_string(),
        24 => "Gnosis".to_string(),
        26 => "Cosmos".to_string(),
        28 => "Osmosis".to_string(),
        30 => "Base".to_string(),
        // Add more chains as they're supported
        _ => format!("Chain-{}", chain_id),
    }
}

/// Format address based on chain format requirements
pub fn format_address_for_chain(chain_id: u16, address: &[u8]) -> String {
    match chain_id {
        // Ethereum and EVM chains use hex with 0x prefix
        2 | 4 | 5 | 6 | 10 | 22 | 23 | 24 | 30 => {
            format!("0x{}", hex::encode(address))
        },
        // Solana uses base58
        1 => {
            bs58::encode(address).into_string()
        },
        // Terra/Cosmos use bech32
        3 | 18 | 26 | 28 => {
            // Simplified for now, would need proper bech32 encoding with prefix
            hex::encode(address)
        },
        // Default to hex encoding without prefix
        _ => hex::encode(address),
    }
}

/// Get token metadata for a given token address
pub fn get_token_metadata(token_mint: &str) -> Option<TokenMetadata> {
    // Initialize cache if not already done
    let cache = TOKEN_METADATA_CACHE.get_or_init(|| {
        let mut map = HashMap::new();
        
        // Add some well-known tokens (these would ideally come from a registry)
        
        // Native SOL
        map.insert(
            "So11111111111111111111111111111111111111112".to_string(),
            TokenMetadata {
                symbol: "SOL".to_string(),
                name: "Solana".to_string(),
                decimals: 9,
            },
        );
        
        // USDC
        map.insert(
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
            TokenMetadata {
                symbol: "USDC".to_string(),
                name: "USD Coin".to_string(),
                decimals: 6,
            },
        );
        
        // USDT
        map.insert(
            "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB".to_string(),
            TokenMetadata {
                symbol: "USDT".to_string(),
                name: "Tether USD".to_string(),
                decimals: 6,
            },
        );

        // WBTC (wrapped)
        map.insert(
            "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh".to_string(),
            TokenMetadata {
                symbol: "WBTC".to_string(),
                name: "Wrapped Bitcoin".to_string(),
                decimals: 8,
            },
        );

        // WETH (wrapped)
        map.insert(
            "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs".to_string(),
            TokenMetadata {
                symbol: "WETH".to_string(),
                name: "Wrapped Ethereum".to_string(),
                decimals: 8,
            },
        );

        // Add more known tokens as needed
        
        map
    });
    
    // Return cloned metadata if found
    cache.get(token_mint).cloned()
}

/// Extract sequence number from log messages
pub fn extract_sequence_from_logs(log_messages: &[&String]) -> u64 {
    for log in log_messages {
        // Look for sequence number in logs
        if log.contains("sequence:") {
            if let Some(seq_part) = log.split("sequence:").nth(1) {
                if let Some(seq_str) = seq_part.trim().split_whitespace().next() {
                    if let Ok(seq) = seq_str.parse::<u64>() {
                        return seq;
                    }
                }
            }
        }
    }
    0 // Default if not found
}

/// Extract sender address from a transaction for a given instruction
pub fn extract_sender_address(transaction: &substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction, inst_idx: u32) -> String {
    if let Some(instructions) = transaction.transaction.as_ref().and_then(|tx| tx.message.as_ref()).map(|m| &m.instructions) {
        if inst_idx < instructions.len() as u32 {
            let instruction = &instructions[inst_idx as usize];
            if let Some(accounts) = transaction.transaction.as_ref().and_then(|tx| tx.message.as_ref()).map(|m| &m.account_keys) {
                if !accounts.is_empty() && !instruction.accounts.is_empty() {
                    // Usually the first account is the payer/sender
                    let sender_idx = instruction.accounts[0] as usize;
                    if sender_idx < accounts.len() {
                        return bs58::encode(&accounts[sender_idx]).into_string();
                    }
                }
            }
        }
    }
    
    "unknown".to_string() // Default if we can't extract
} 