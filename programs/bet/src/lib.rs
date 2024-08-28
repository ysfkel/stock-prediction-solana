// mod contract;
mod state;
mod constants;
mod utils;
mod error;

use anchor_lang::{
    prelude::*, 
    system_program
};
use pyth_sdk_solana::load_price_feed_from_account_info;
use state::*;
use crate::constants::*;
use utils::get_unix_timestamp;
declare_id!("4BURrvZqxAFbCLRRjfVxjK6yXetkRSPoUQLQZeCMwMZK");

#[program]
pub mod prediction {
    use error::BetError;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, master: Master) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_bet(ctx: Context<CreateBet>, amount: u64, price: f64, duration: u32, pyth_sdk_solana: Pubkey) -> Result<()> { 
        let master = &mut ctx.accounts.master;
        let bet = &mut ctx.accounts.bet;
        master.last_bet_id += 1;
        bet.id = master.last_bet_id;
        bet.amount = amount;
        bet.pyth_price_key = pyth_sdk_solana;
        bet.expiryts = get_unix_timestamp()? + duration as i64;
        bet.prediction_a = BetPrediction {
            player: ctx.accounts.player.key(),
            price,
        };

        // Transfer the SOL amount to the bet PDA  
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.player.to_account_info(),
                    to: bet.to_account_info(),
                }
            ),
            amount
        )?;
        Ok(())   
    }

    pub fn enter_bet(ctx: Context<EnterBet>, price: f64) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        bet.predidction_b = Some(BetPrediction{
            player: ctx.accounts.player.key(),
            price,
        });
        bet.state = BetState::Started;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer { 
                    from: ctx.accounts.player.to_account_info(),
                    to: bet.to_account_info(),
                 },
            ),
            bet.amount
        )?;

        Ok(())
    }

    pub fn claim_bet(ctx: Context<ClaimBet>) -> Result<()> {
        
        let bet = &mut ctx.accounts.bet;
        let prize = bet.amount.checked_mul(2).unwrap();
        
        let pyth_account_info = &ctx.accounts.pyth;
        **bet.to_account_info().try_borrow_mut_lamports()? -= prize;
        let feed = load_price_feed_from_account_info(pyth_account_info)
        .map_err(|_|error!(BetError::InvalidPythAccount))?;
       
       let price_data = feed.get_price_unchecked();

       require!(price_data.price <= f64::MAX as i64, BetError::PriceTooBig);
       let pyth_price = price_data.price as f64;
       msg!("Pyth price is: {}", pyth_price);

       // Adjust prices to compare them with pyths price 
       // Real price = Pyth price * 10 (Pyth Exponent)

       let multiplier = 10f64.powi(-price_data.expo);
       let adjusted_player_a = bet.prediction_a.price * multiplier;
       let adjusted_player_b = bet.predidction_b.as_ref().unwrap().price * multiplier;
       msg!("Adjustd player A prediction: {}", adjusted_player_a);
       msg!("Adjustd player B prediction: {}", adjusted_player_b);

       let abs_player_a = (pyth_price - adjusted_player_a).abs();
       let abs_player_b = (pyth_price - adjusted_player_b).abs();
       if abs_player_a < abs_player_b {
          msg!("Winner is Player A, sending {} lamports", prize);
          bet.state = BetState::PlayerAWon;
          **ctx.accounts.player_a.to_account_info()
          .try_borrow_mut_lamports()? +=prize;
       } else if abs_player_b < abs_player_a {
            msg!("Winner is Player B, sending {} lamports", prize);
            bet.state = BetState::PlayerBWon;
            **ctx.accounts.player_b.to_account_info()
            .try_borrow_mut_lamports()? +=prize;
       } else {
         let draw_amount = bet.amount;
         msg!("Draw! Sending both player {} lamports", draw_amount);
         bet.state = BetState::Draw;
         // Return both players amount back
         **ctx.accounts.player_a.to_account_info()
         .try_borrow_mut_lamports()? += draw_amount;

         **ctx.accounts.player_b.to_account_info()
         .try_borrow_mut_lamports()? += draw_amount;

       }

        Ok(())
    }

    pub fn close_bet(ctx: Context<CloseBet>) -> Result<()> {
        
        Ok(())
    }
}
 