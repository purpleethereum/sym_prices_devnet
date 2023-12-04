use crate::*;

#[account(zero_copy(unsafe))]
pub struct MyProgramState {
    pub bump: u8,
    pub authority: Pubkey,
    pub function: Pubkey,
}

#[repr(packed)]
#[zero_copy(unsafe)]
pub struct OracleData {
    pub oracle_timestamp: u128,
    pub prices: [u64; PRICES_SIZE],
}

#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct MyOracleState {
    pub bump: u8,
    pub prices: [u64; PRICES_SIZE],
    pub timestamps: [u64; PRICES_SIZE],
}

impl MyOracleState {
    pub fn save_rows(
        &mut self,
        fetch_timestamp: u128,
        prices: [u64; PRICES_SIZE],
    ) -> anchor_lang::Result<()> {
        let timestamp = Clock::get()?.unix_timestamp.try_into().unwrap();
        for i in 0usize..PRICES_SIZE {
            if prices[i].ge(&0) {
                self.prices[i] = prices[i];
                self.timestamps[i] = timestamp;
            }
        }
        let price = self.prices[0];
        msg!("Oracle Time: {}", timestamp);
        msg!("Fetch Time: {}", fetch_timestamp);
        msg!("Price 0: {}", price);
        Ok(())
    }
    
}