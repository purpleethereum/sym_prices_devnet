pub use switchboard_solana::prelude::*;

pub mod error;
pub use error::*;

pub mod model;
pub use model::*;

pub mod utils;
pub use utils::*;

pub mod actions;
pub use actions::*;

declare_id!("heD5N6R8z9TgJJHrDJtnPwj9EsZvrYtHFbxhFjyjb1p");

pub const PROGRAM_SEED: &[u8] = b"PROG_SEED";

pub const ORACLE_SEED: &[u8] = b"ORAC_SEED";

pub const PRICES_SIZE: usize = 40;

#[program]
pub mod sym_prices_v2 {

    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn initialize(
        ctx: Context<Initialize>,
        params: InitializeParams,
    ) -> anchor_lang::Result<()> {
        Initialize::actuate(&ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &fetch_timestamp, &prices))]
    pub fn refresh_oracles(
        ctx: Context<RefreshPrices>,
        fetch_timestamp: u128,
        prices: [u64; PRICES_SIZE],
    ) -> anchor_lang::Result<()> {
        RefreshPrices::actuate(
            &ctx, 
            fetch_timestamp,
            prices
        )
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn set_function(
        ctx: Context<SetFunction>,
        params: SetFunctionParams,
    ) -> anchor_lang::Result<()> {
        SetFunction::actuate(&ctx, &params)
    }
    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn trigger_function(
    //     ctx: Context<TriggerFunction>,
    //     params: TriggerFunctionParams,
    // ) -> anchor_lang::Result<()> {
    //     TriggerFunction::actuate(&ctx, &params)
    // }
    pub fn load_prices(
        ctx: Context<LoadPrices>,
        index: u8
    ) -> anchor_lang::Result<PriceWithTimestamp> {
        LoadPrices::load(&ctx, index)
    }

    pub fn close_accounts(
        _ctx: Context<CloseAccounts>
    )-> anchor_lang::Result<()> { Ok(()) }
    
}