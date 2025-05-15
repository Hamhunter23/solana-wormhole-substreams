// @generated
pub mod sf {
    pub mod solana {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.solana.type.v1)
            pub mod v1 {
                include!("sf.solana.type.v1.rs");
                // @@protoc_insertion_point(sf.solana.type.v1)
            }
        }
    }
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod solana {
            // @@protoc_insertion_point(attribute:sf.substreams.solana.v1)
            pub mod v1 {
                include!("sf.substreams.solana.v1.rs");
                // @@protoc_insertion_point(sf.substreams.solana.v1)
            }
        }
    }
}
pub mod substreams {
    pub mod v1 {
        // @@protoc_insertion_point(attribute:substreams.v1.program)
        pub mod program {
            include!("substreams.v1.program.rs");
            // @@protoc_insertion_point(substreams.v1.program)
        }
    }
}
pub mod wormhole {
    pub mod combined {
        // @@protoc_insertion_point(attribute:wormhole.combined.v1)
        pub mod v1 {
            include!("wormhole.combined.v1.rs");
            // @@protoc_insertion_point(wormhole.combined.v1)
        }
    }
    pub mod nft_bridge {
        // @@protoc_insertion_point(attribute:wormhole.nft_bridge.v1)
        pub mod v1 {
            include!("wormhole.nft_bridge.v1.rs");
            // @@protoc_insertion_point(wormhole.nft_bridge.v1)
        }
    }
    pub mod output {
        // @@protoc_insertion_point(attribute:wormhole.output.v1)
        pub mod v1 {
            include!("wormhole.output.v1.rs");
            // @@protoc_insertion_point(wormhole.output.v1)
        }
    }
    pub mod token_bridge {
        // @@protoc_insertion_point(attribute:wormhole.token_bridge.v1)
        pub mod v1 {
            include!("wormhole.token_bridge.v1.rs");
            // @@protoc_insertion_point(wormhole.token_bridge.v1)
        }
    }
}
