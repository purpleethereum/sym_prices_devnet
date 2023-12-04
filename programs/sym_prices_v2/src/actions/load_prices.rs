use crate::*;
use anchor_lang::prelude::{AccountInfo,AccountLoader};
use std::cell::RefMut;


#[derive(Accounts)]
pub struct LoadPrices<'info> {
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle.load()?.bump
    )]
    pub oracle: AccountLoader<'info, MyOracleState>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct LoadPricesParams {
    
}



#[derive(Default, Debug, Copy, Clone, AnchorSerialize, AnchorDeserialize)]
#[repr(C)]
pub struct PriceWithTimestamp {
    pub price: u64,
    pub write_timestamp: u64,
}




impl LoadPrices<'_> {
    pub fn load<'a>(ctx: &Context<Self>,  index: u8) -> anchor_lang::Result<PriceWithTimestamp> {
        let account_info = &mut ctx.accounts.oracle.to_account_info();
        Self::get_prices(account_info, index)    
    }

    pub fn get_prices<'a>(oracle_state: &'a AccountInfo, index: u8) -> anchor_lang::Result<PriceWithTimestamp> {
        
        let account_data: RefMut<'a, [u8]> = RefMut::map(oracle_state.try_borrow_mut_data().unwrap(), |data| *data);

        let price: [u8; 8] = account_data[(index as usize) * 8 + 9..((index as usize) + 1) * 8 + 9].try_into().unwrap();
        let t: [u8; 8] = account_data[(index as usize) * 8 + 8 * PRICES_SIZE + 9..((index as usize) + 1) * 8 + 8 * PRICES_SIZE + 9].try_into().unwrap();
        
        let write_timestamp: u64 = u64::from_le_bytes(t);
        let mantissa: u64 = u64::from_le_bytes(price);

        msg!("price {}", mantissa);
        msg!("writing timestamp {}", write_timestamp);
        Ok(PriceWithTimestamp { 
            price: mantissa, 
            write_timestamp: write_timestamp, 
        })
    }

    
}
