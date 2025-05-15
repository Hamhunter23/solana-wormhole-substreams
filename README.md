# Wormhole Solana Indexer

A comprehensive Substreams-based indexer for Wormhole cross-chain messaging and asset transfers on Solana.

## Overview

This project indexes Wormhole's ecosystem on Solana, tracking:

- Core Bridge message publications
- Token Bridge transfers (with real data parsing)
- NFT Bridge transfers
- Wormhole Program activities

The indexer provides rich analytics about cross-chain activity, chain pair interactions, and token metrics.

## Programs Indexed

| Program | Address | Description |
|---------|---------|-------------|
| Core Bridge | `worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth` | Handles message publications and verifications |
| Token Bridge | `wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb` | Manages token transfers between chains |
| NFT Bridge | `WnFt12ZrnzZrFZkt2xsNsaNWoQribnuQ5B5FrDbwDhD` | Manages NFT transfers between chains |
| Wormhole Program | `HDwcJBJXjL9FpJ7UBsYBtaDjsBUhuLCUYoz3zr8SWWaQ` | Main Wormhole program |

## Key Features

- **Real Anchor IDL Data Parsing**: Uses proper Anchor deserialization for TransferOut, TransferIn, and TransferNative events
- **Cross-Chain Address Formatting**: Formats addresses according to chain standards (hex for EVM, base58 for Solana)
- **Token Metadata Resolution**: Includes symbol, name, and decimals for major tokens (SOL, USDC, USDT, WBTC, WETH)
- **Chain ID Mapping**: Maps numeric chain IDs (1, 2, etc.) to human-readable names (Solana, Ethereum, etc.)
- **Advanced Analytics**: Calculates TVL (Total Value Locked), DAU (Daily Active Users), and token volume metrics

## Modules

| Module | Description |
|--------|-------------|
| `map_core_bridge_data` | Extracts MessagePublication events from the Core Bridge |
| `map_token_bridge_data` | Extracts token transfer events from the Token Bridge |
| `map_nft_bridge_data` | Extracts NFT transfer events from the NFT Bridge |
| `map_wormhole_program_data` | Extracts message events from the main Wormhole program |
| `combine_wormhole_activity` | Combines data from all bridges to provide rich analytics |

## Data Model

### Core Messages

Track cross-chain messages with:
- Transaction and block information
- Emitter account and sequence number
- Consistency level and payload data

### Token Transfers

Track token movements with:
- Source and destination chain information
- Token details (address, symbol, name, decimals)
- Transfer amount and fees
- Sender and recipient addresses
- Properly formatted addresses for each blockchain

### NFT Transfers

Track NFT movements with:
- Source and destination chain information
- NFT details (address, token ID, URI, name, symbol)
- Sender and recipient addresses

### Combined Analytics

Provides aggregated insights:
- Cross-chain value flows and volume
- Chain pair interaction statistics
- Top tokens and most active pairs
- User activity metrics
- Total Value Locked (TVL) calculations
- Daily Active Users (DAU) tracking

## Supported Chains

The indexer supports all chains in the Wormhole ecosystem, including:

| Chain ID | Name |
|----------|------|
| 1 | Solana |
| 2 | Ethereum |
| 4 | BSC |
| 5 | Polygon |
| 6 | Avalanche |
| 10 | Fantom |
| 22 | Arbitrum |
| 23 | Optimism |
| 30 | Base |

... and many more!

## Token Bridge Event Types

The indexer handles three main event types from the Token Bridge:

1. **TransferOut**: Tokens leaving Solana to another chain
2. **TransferIn**: Tokens arriving to Solana from another chain
3. **TransferNative**: Native SOL transfers to other chains

Each event type is properly deserialized using Anchor's IDL definitions.

## Usage with GraphQL

Example queries to extract insights from the indexed data:

```graphql
# Get recent token transfers
query {
  tokenTransfers(limit: 10, orderBy: BLOCK_TIMESTAMP_DESC) {
    txId
    blockTimestamp
    fromAddress
    toAddress
    tokenSymbol
    amount
    toChain
  }
}

# Get chain pair activity
query {
  chainPairs(orderBy: TOKEN_TRANSFER_COUNT_DESC, limit: 5) {
    sourceChain
    destinationChain
    messageCount
    tokenTransferCount
    nftTransferCount
    tokenVolumeUsd
  }
}
```

## Building and Running

Build the Substreams:

```bash
cargo build --target wasm32-unknown-unknown --release
substreams pack
```

Deploy the package to Substreams.dev:

```bash
substreams deploy ./wormhole-indexer-v0.1.0.spkg
```

Run the Substreams:

```bash
substreams run -e <ENDPOINT> map_token_bridge_data -s <START_BLOCK> -t <STOP_BLOCK>
substreams run -e <ENDPOINT> combine_wormhole_activity -s <START_BLOCK> -t <STOP_BLOCK>
```

## License

MIT 