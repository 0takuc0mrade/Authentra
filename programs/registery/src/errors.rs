use anchor_lang::prelude::*;

#[error_code]
pub enum RegistryError {
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Invalid content data")]
    InvalidContent,
}
