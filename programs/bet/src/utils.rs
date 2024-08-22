use anchor_lang::{
   prelude::*,
   solana_program::clock::{
    Clock, UnixTimestamp
   },
};

use crate::{constants::*, state::*};

pub fn get_unix_timestamp() -> Result<UnixTimestamp> {
    let clock = Clock::get()?;
    Ok(clock.unix_timestamp)
}

pub fn validate_enter_bet(bet: &Bet) -> bool {
    bet.predidction_b.is_none() && (bet.expiryts - MINIMUM_REMAINING_TIME_UNTIL_EXPIRY > get_unix_timestamp().unwrap())
}

pub fn validate_claim_bet(bet: &Bet) -> bool {
    match bet.state {
        BetState::Started => {
            let current_ts = get_unix_timestamp().unwrap();
            let time_passed_since_expiry = current_ts - bet.expiryts;
            0 < time_passed_since_expiry && time_passed_since_expiry <= MAXIMUM_CLAIMABLE_PERIOD
        },
        BetState::PlayerAWon => false,
        BetState::PlayerBWon => false,
        BetState::Created => false,
        BetState::Draw => false,
    }
}