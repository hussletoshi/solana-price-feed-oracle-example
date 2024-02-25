use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;
use std::str::FromStr;



declare_id!("11111111111111111111111111111111"); // Your program ID

const BTC_USDC_FEED: &str = "HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J"; // The oracle pubkeys to ensure that the key provided is the actual one in order to avoid malicious activity
const ETH_USDC_FEED: &str = "EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw"; // The oracle pubkeys to ensure that the key provided is the actual one in order to avoid malicious activity

// List of pyth data feeds --> https://pyth.network/developers/price-feed-ids#solana-devnet (devnet)

const STALENESS_THRESHOLD: u64 = 60; // staleness threshold in seconds

#[program]
mod hello_pyth {
    use super::*;
    pub fn fetch_btc_price(ctx: Context<FetchBitcoinPrice>) -> Result<()> {
        let price_account_info = &ctx.accounts.price_feed; 
        let price_feed = load_price_feed_from_account_info(&price_account_info).unwrap();  // Use Pyth SDK to fetch Oracle price feed
        let current_timestamp = Clock::get()?.unix_timestamp; // Get current solana time to make sure that quote is recent and not outdated.
        let current_price = price_feed
            .get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD) // Gets prices using SDK and ensure that it has been updated recently
            .unwrap();
        require!(current_price.price != 0, FeedError::OraclePreviousUpdateFailed);

        let display_price = u64::try_from(current_price.price).unwrap()
            / 10u64.pow(u32::try_from(-current_price.expo).unwrap());
    
        msg!(
            "BTC/USD price: ({})",
            display_price
        );

        Ok(())
    }

    pub fn fetch_eth_price(ctx: Context<FetchEtherPrice>) -> Result<()> {


        let price_account_info = &ctx.accounts.price_feed_eth;
        let price_feed = load_price_feed_from_account_info(&price_account_info).unwrap(); // Use Pyth SDK to fetch Oracle price feed
        let current_timestamp = Clock::get()?.unix_timestamp; // Get current solana time to make sure that quote is recent and not outdated.
        let current_price = price_feed
            .get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD) // Gets prices using SDK and ensure that it has been updated recently
            .unwrap();
        require!(current_price.price != 0, FeedError::OraclePreviousUpdateFailed); // Ensure that oracle has provided the correct number and no errors have occured


        let display_price = u64::try_from(current_price.price).unwrap()
            / 10u64.pow(u32::try_from(-current_price.expo).unwrap()); // Price is given in a 12 figure format. Dividing by exponent provided to get human number.


        // Confidence
        // Think of confidence as the wiggle room the prices has to be accurate like a possible loss. 
        // Meaning that if the confidence is 50 
        // and the price is 100 
        // The price may actually range from 50-150
        // 100 +- 50
        let display_confidence = u64::try_from(current_price.conf).unwrap()
            / 10u64.pow(u32::try_from(-current_price.expo).unwrap());
        

        msg!(
            "ETH/USD price: ({})",
            display_price
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FetchBitcoinPrice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // Ensure that oracle has provided the correct number and no errors have occured
    /// CHECK: Check during execution of tx
    #[account(address = Pubkey::from_str(BTC_USDC_FEED).unwrap() @ FeedError::InvalidPriceFeed)]
    pub price_feed: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FetchEtherPrice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Check during execution of tx
    #[account(address = Pubkey::from_str(ETH_USDC_FEED).unwrap() @ FeedError::InvalidPriceFeed)]
    pub price_feed_eth: AccountInfo<'info>,
}

#[error_code]
pub enum FeedError {
    #[msg("Invalid Price Feed")]
    InvalidPriceFeed,
    #[msg("Price Feed has expired")]
    OraclePreviousUpdateFailed,
}
