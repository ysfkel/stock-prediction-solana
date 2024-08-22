use anchor_lang::solana_program::clock::UnixTimestamp;


pub const MASTER_SEED: &[u8] = b"master";
pub const BET_SEED: &[u8] = b"bet";

pub const MINIMUM_REMAINING_TIME_UNTIL_EXPIRY: UnixTimestamp = 120; /* we will not allow users to enter a bit in the last 20 seconds*/

pub const MAXIMUM_CLAIMABLE_PERIOD: UnixTimestamp = 300;