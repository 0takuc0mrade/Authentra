use anchor_lang::prelude::*;

pub fn validate_uri(uri: &str) -> bool {
    uri.starts_with("ipfs://") || uri.starts_with("https://")
}
