pub mod idl {
    // Unused import, removing it
}

pub mod program {
    pub mod events {
        use anchor_lang::{AnchorDeserialize, Discriminator};
        use std::io::{self, Read};

        // Define the MessagePublication event structure
        #[derive(Clone)]
        pub struct MessagePublication {
            pub nonce: u32,
            pub payload: Vec<u8>,
            pub emitter_account: anchor_lang::prelude::Pubkey,
            pub sequence: u64,
            pub consistency_level: u8,
            pub timestamp: u64,
        }

        // Implement AnchorDeserialize manually to avoid borsh issues
        impl AnchorDeserialize for MessagePublication {
            fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
                use std::io::{Error, ErrorKind};
                
                // Read nonce (u32)
                let nonce = match u32::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize nonce")),
                };
                
                // Read payload length and data (Vec<u8>)
                let payload_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize payload length")),
                };
                
                if buf.len() < payload_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for payload"));
                }
                
                let payload = buf[..payload_len].to_vec();
                *buf = &buf[payload_len..];
                
                // Read emitter_account (Pubkey)
                let mut emitter_bytes = [0u8; 32];
                if buf.len() < 32 {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for emitter_account"));
                }
                emitter_bytes.copy_from_slice(&buf[..32]);
                *buf = &buf[32..];
                let emitter_account = anchor_lang::prelude::Pubkey::new_from_array(emitter_bytes);
                
                // Read sequence (u64)
                let sequence = match u64::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize sequence")),
                };
                
                // Read consistency_level (u8)
                let consistency_level = match u8::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize consistency_level")),
                };
                
                // Read timestamp (u64)
                let timestamp = match u64::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize timestamp")),
                };
                
                Ok(MessagePublication {
                    nonce,
                    payload,
                    emitter_account,
                    sequence,
                    consistency_level,
                    timestamp,
                })
            }
            
            fn deserialize_reader<R: Read>(_reader: &mut R) -> io::Result<Self> {
                // Simplified implementation for now
                Err(io::Error::new(io::ErrorKind::Unsupported, "Reader deserialization not implemented for MessagePublication"))
            }
        }

        impl Discriminator for MessagePublication {
            // Fix the return type to &'static [u8]
            const DISCRIMINATOR: &'static [u8] = &[61, 126, 136, 199, 141, 182, 25, 218];
        }
    }
}

pub mod token_bridge {
    pub mod events {
        use anchor_lang::{AnchorDeserialize, Discriminator};
        use std::io::{self, Read};

        // TransferOut event structure for tokens leaving Solana
        #[derive(Clone)]
        pub struct TransferOut {
            pub amount: u64,
            pub token_address: [u8; 32],  // Solana token mint as bytes
            pub token_chain: u16,         // Source chain ID (1 = Solana)
            pub recipient_address: Vec<u8>, // Recipient address on target chain
            pub recipient_chain: u16,     // Destination chain ID
            pub fee: u64,                 // Transfer fee amount
            pub nonce: u32,               // Random number for uniqueness
        }

        // Implement AnchorDeserialize for TransferOut
        impl AnchorDeserialize for TransferOut {
            fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
                use std::io::{Error, ErrorKind};
                
                // Read amount (u64)
                let amount = match u64::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize amount")),
                };
                
                // Read token_address ([u8; 32])
                let mut token_address = [0u8; 32];
                if buf.len() < 32 {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for token_address"));
                }
                token_address.copy_from_slice(&buf[..32]);
                *buf = &buf[32..];
                
                // Read token_chain (u16)
                let token_chain = match u16::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize token_chain")),
                };
                
                // Read recipient_chain (u16)
                let recipient_chain = match u16::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize recipient_chain")),
                };
                
                // Read recipient_address length and data (Vec<u8>)
                let recipient_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize recipient_address length")),
                };
                
                if buf.len() < recipient_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for recipient_address"));
                }
                
                let recipient_address = buf[..recipient_len].to_vec();
                *buf = &buf[recipient_len..];
                
                // Read fee (u64)
                let fee = match u64::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize fee")),
                };
                
                // Read nonce (u32)
                let nonce = match u32::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize nonce")),
                };
                
                Ok(TransferOut {
                    amount,
                    token_address,
                    token_chain,
                    recipient_address,
                    recipient_chain,
                    fee,
                    nonce,
                })
            }
            
            fn deserialize_reader<R: Read>(_reader: &mut R) -> io::Result<Self> {
                // Simplified implementation for now
                Err(io::Error::new(io::ErrorKind::Unsupported, "Reader deserialization not implemented for TransferOut"))
            }
        }

        impl Discriminator for TransferOut {
            const DISCRIMINATOR: &'static [u8] = &[57, 138, 223, 97, 127, 182, 171, 26];
        }

        // TransferIn event structure for tokens arriving to Solana
        #[derive(Clone)]
        pub struct TransferIn {
            pub amount: u64,
            pub token_address: [u8; 32],  // Token mint as bytes
            pub token_chain: u16,         // Source chain ID
            pub sender_address: Vec<u8>,  // Sender address on source chain
            pub sender_chain: u16,        // Source chain ID
            pub nonce: u32,               // Random number for uniqueness
        }

        impl AnchorDeserialize for TransferIn {
            fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
                use std::io::{Error, ErrorKind};
                
                // Read amount (u64)
                let amount = match u64::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize amount")),
                };
                
                // Read token_address ([u8; 32])
                let mut token_address = [0u8; 32];
                if buf.len() < 32 {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for token_address"));
                }
                token_address.copy_from_slice(&buf[..32]);
                *buf = &buf[32..];
                
                // Read token_chain (u16)
                let token_chain = match u16::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize token_chain")),
                };
                
                // Read sender_chain (u16)
                let sender_chain = match u16::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize sender_chain")),
                };
                
                // Read sender_address length and data (Vec<u8>)
                let sender_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize sender_address length")),
                };
                
                if buf.len() < sender_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for sender_address"));
                }
                
                let sender_address = buf[..sender_len].to_vec();
                *buf = &buf[sender_len..];
                
                // Read nonce (u32)
                let nonce = match u32::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize nonce")),
                };
                
                Ok(TransferIn {
                    amount,
                    token_address,
                    token_chain,
                    sender_address,
                    sender_chain,
                    nonce,
                })
            }
            
            fn deserialize_reader<R: Read>(_reader: &mut R) -> io::Result<Self> {
                // Simplified implementation for now
                Err(io::Error::new(io::ErrorKind::Unsupported, "Reader deserialization not implemented for TransferIn"))
            }
        }

        impl Discriminator for TransferIn {
            const DISCRIMINATOR: &'static [u8] = &[18, 144, 51, 127, 228, 152, 108, 36];
        }

        // TransferNative event structure for native SOL transfers
        #[derive(Clone)]
        pub struct TransferNative {
            pub amount: u64,
            pub recipient_address: Vec<u8>,
            pub recipient_chain: u16,
            pub fee: u64,
            pub nonce: u32,
        }

        impl AnchorDeserialize for TransferNative {
            fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
                use std::io::{Error, ErrorKind};
                
                // Read amount (u64)
                let amount = match u64::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize amount")),
                };
                
                // Read recipient_chain (u16)
                let recipient_chain = match u16::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize recipient_chain")),
                };
                
                // Read recipient_address length and data (Vec<u8>)
                let recipient_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize recipient_address length")),
                };
                
                if buf.len() < recipient_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for recipient_address"));
                }
                
                let recipient_address = buf[..recipient_len].to_vec();
                *buf = &buf[recipient_len..];
                
                // Read fee (u64)
                let fee = match u64::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize fee")),
                };
                
                // Read nonce (u32)
                let nonce = match u32::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize nonce")),
                };
                
                Ok(TransferNative {
                    amount,
                    recipient_address,
                    recipient_chain,
                    fee,
                    nonce,
                })
            }
            
            fn deserialize_reader<R: Read>(_reader: &mut R) -> io::Result<Self> {
                // Simplified implementation for now
                Err(io::Error::new(io::ErrorKind::Unsupported, "Reader deserialization not implemented for TransferNative"))
            }
        }

        impl Discriminator for TransferNative {
            const DISCRIMINATOR: &'static [u8] = &[149, 73, 42, 180, 65, 148, 103, 53];
        }
    }
}

pub mod nft_bridge {
    pub mod events {
        use anchor_lang::{AnchorDeserialize, Discriminator};
        use std::io::{self, Read};

        // NFTTransfer event structure for NFTs leaving Solana
        #[derive(Clone)]
        pub struct NFTTransfer {
            pub token_id: u64,
            pub token_address: [u8; 32],  // Solana NFT mint as bytes
            pub token_chain: u16,         // Source chain ID (1 = Solana)
            pub recipient_address: Vec<u8>, // Recipient address on target chain
            pub recipient_chain: u16,     // Destination chain ID
            pub nonce: u32,               // Random number for uniqueness
            pub uri: Vec<u8>,             // NFT URI
            pub name: Vec<u8>,            // NFT name
            pub symbol: Vec<u8>,          // NFT symbol
        }

        // Implement AnchorDeserialize for NFTTransfer
        impl AnchorDeserialize for NFTTransfer {
            fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
                use std::io::{Error, ErrorKind};
                
                // Read token_id (u64)
                let token_id = match u64::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize token_id")),
                };
                
                // Read token_address ([u8; 32])
                let mut token_address = [0u8; 32];
                if buf.len() < 32 {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for token_address"));
                }
                token_address.copy_from_slice(&buf[..32]);
                *buf = &buf[32..];
                
                // Read token_chain (u16)
                let token_chain = match u16::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize token_chain")),
                };
                
                // Read recipient_chain (u16)
                let recipient_chain = match u16::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize recipient_chain")),
                };
                
                // Read recipient_address length and data (Vec<u8>)
                let recipient_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize recipient_address length")),
                };
                
                if buf.len() < recipient_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for recipient_address"));
                }
                
                let recipient_address = buf[..recipient_len].to_vec();
                *buf = &buf[recipient_len..];
                
                // Read nonce (u32)
                let nonce = match u32::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize nonce")),
                };
                
                // Read URI length and data (Vec<u8>)
                let uri_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize URI length")),
                };
                
                if buf.len() < uri_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for URI"));
                }
                
                let uri = buf[..uri_len].to_vec();
                *buf = &buf[uri_len..];
                
                // Read name length and data (Vec<u8>)
                let name_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize name length")),
                };
                
                if buf.len() < name_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for name"));
                }
                
                let name = buf[..name_len].to_vec();
                *buf = &buf[name_len..];
                
                // Read symbol length and data (Vec<u8>)
                let symbol_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize symbol length")),
                };
                
                if buf.len() < symbol_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for symbol"));
                }
                
                let symbol = buf[..symbol_len].to_vec();
                *buf = &buf[symbol_len..];
                
                Ok(NFTTransfer {
                    token_id,
                    token_address,
                    token_chain,
                    recipient_address,
                    recipient_chain,
                    nonce,
                    uri,
                    name,
                    symbol,
                })
            }
            
            fn deserialize_reader<R: Read>(_reader: &mut R) -> io::Result<Self> {
                // Simplified implementation for now
                Err(io::Error::new(io::ErrorKind::Unsupported, "Reader deserialization not implemented for NFTTransfer"))
            }
        }

        impl Discriminator for NFTTransfer {
            const DISCRIMINATOR: &'static [u8] = &[233, 146, 209, 97, 112, 27, 49, 37];
        }

        // NFTReceive event structure for NFTs arriving to Solana
        #[derive(Clone)]
        pub struct NFTReceive {
            pub token_id: u64,
            pub token_address: [u8; 32],  // Token mint as bytes
            pub token_chain: u16,         // Source chain ID
            pub sender_address: Vec<u8>,  // Sender address on source chain
            pub sender_chain: u16,        // Source chain ID
            pub nonce: u32,               // Random number for uniqueness
            pub uri: Vec<u8>,             // NFT URI
            pub name: Vec<u8>,            // NFT name
            pub symbol: Vec<u8>,          // NFT symbol
        }

        impl AnchorDeserialize for NFTReceive {
            fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
                use std::io::{Error, ErrorKind};
                
                // Read token_id (u64)
                let token_id = match u64::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize token_id")),
                };
                
                // Read token_address ([u8; 32])
                let mut token_address = [0u8; 32];
                if buf.len() < 32 {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for token_address"));
                }
                token_address.copy_from_slice(&buf[..32]);
                *buf = &buf[32..];
                
                // Read token_chain (u16)
                let token_chain = match u16::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize token_chain")),
                };
                
                // Read sender_chain (u16)
                let sender_chain = match u16::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize sender_chain")),
                };
                
                // Read sender_address length and data (Vec<u8>)
                let sender_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize sender_address length")),
                };
                
                if buf.len() < sender_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for sender_address"));
                }
                
                let sender_address = buf[..sender_len].to_vec();
                *buf = &buf[sender_len..];
                
                // Read nonce (u32)
                let nonce = match u32::deserialize(buf) {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize nonce")),
                };
                
                // Read URI length and data (Vec<u8>)
                let uri_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize URI length")),
                };
                
                if buf.len() < uri_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for URI"));
                }
                
                let uri = buf[..uri_len].to_vec();
                *buf = &buf[uri_len..];
                
                // Read name length and data (Vec<u8>)
                let name_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize name length")),
                };
                
                if buf.len() < name_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for name"));
                }
                
                let name = buf[..name_len].to_vec();
                *buf = &buf[name_len..];
                
                // Read symbol length and data (Vec<u8>)
                let symbol_len = match u32::deserialize(buf) {
                    Ok(val) => val as usize,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to deserialize symbol length")),
                };
                
                if buf.len() < symbol_len {
                    return Err(Error::new(ErrorKind::InvalidData, "Buffer too short for symbol"));
                }
                
                let symbol = buf[..symbol_len].to_vec();
                *buf = &buf[symbol_len..];
                
                Ok(NFTReceive {
                    token_id,
                    token_address,
                    token_chain,
                    sender_address,
                    sender_chain,
                    nonce,
                    uri,
                    name,
                    symbol,
                })
            }
            
            fn deserialize_reader<R: Read>(_reader: &mut R) -> io::Result<Self> {
                // Simplified implementation for now
                Err(io::Error::new(io::ErrorKind::Unsupported, "Reader deserialization not implemented for NFTReceive"))
            }
        }

        impl Discriminator for NFTReceive {
            const DISCRIMINATOR: &'static [u8] = &[101, 37, 152, 215, 44, 55, 173, 14];
        }
    }
}