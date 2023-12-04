use crate::*;
#[derive(Accounts)]
pub struct RefreshPrices<'info> {
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle.load()?.bump
    )]
    pub oracle: AccountLoader<'info, MyOracleState>,

    // #[account(
    //     constraint =
    //         function.load()?.validate(
    //           &enclave_signer.to_account_info()
    //         )? @ BasicOracleError::FunctionValidationFailed
    // )]
    #[account(
        constraint = function.load()?.validate_routine(
            &routine,
            &enclave_signer.to_account_info(),
        )?
    )]
    pub function: AccountLoader<'info, FunctionAccountData>,
    #[account(
        has_one = function,
    )]
    pub routine: Box<Account<'info, FunctionRoutineAccountData>>,
    pub enclave_signer: Signer<'info>,

}



impl RefreshPrices<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _fetch_timestamp: &u128,
        _prices: &[u64; PRICES_SIZE],
    ) -> anchor_lang::Result<()> {
        Ok(())
    }

    pub fn actuate(
        ctx: &Context<Self>,
        fetch_timestamp: u128,
        prices: [u64; PRICES_SIZE],
    ) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;
        oracle.save_rows(fetch_timestamp, prices)?;
        Ok(())
    }
}
