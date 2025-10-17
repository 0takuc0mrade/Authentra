use anchor_lang::prelude::*;

#[error_code]
pub enum LicenseError {
    #[msg("License config already initialized")]
    AlreadyInitialized,

    #[msg("Invalid authority for this operation")]
    InvalidAuthority,

    #[msg("Insufficient funds for transaction")]
    InsufficientFunds,

    #[msg("License is not active")]
    LicenseInactive,

    #[msg("Mint manager is currently in use")]
    MintManagerInUse,

    #[msg("License has expired")]
    LicenseExpired,

    #[msg("Invalid license configuration")]
    InvalidConfig,

    #[msg("Invalid mint manager authority")]
    InvalidMintManagerAuthority,

    #[msg("Payment amount does not match license fee")]
    InvalidPaymentAmount,

    #[msg("License already exists for this user and content")]
    LicenseAlreadyExists,

    #[msg("License verification failed")]
    VerificationFailed,

    #[msg("Invalid licensee for this license")]
    InvalidLicensee,
}
