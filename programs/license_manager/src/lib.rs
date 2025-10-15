pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

use anchor_lang::prelude::*;

declare_id!("46768WTgs123tGwaS1XJ4dVd8eSUheh3rSvzkuBVM4y5");

#[program]
pub mod license_manager {
    use super::*;

    pub fn initialize_license_config(ctx: Context<InitializeConfig>) -> Result<()> {
        instructions::initialize_config::initialize_license_config_handler(ctx)
    }
}
