use anchor_lang::prelude::*;

use crate::constants::*;
use crate::utils::*;
use crate::error::BetError;

#[account]
pub struct Master {
    pub last_bet_id: u64,
}

#[account]
pub struct Bet {
    pub id: u64,
    pub amount: u64,
    pub prediction_a: BetPrediction,
    pub predidction_b: Option<BetPrediction>,
    pub state: BetState,
    pub pyth_price_key: Pubkey,
    pub expiryts: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct  BetPrediction {
    pub player: Pubkey,
    pub price: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BetState {
    Created, 
    Started,
    PlayerAWon,
    PlayerBWon,
    Draw,
}


//////////////////////////////// Context //////////////////// 
#[derive(Accounts)]
pub struct Initialize<'info> {
   
   #[account(
      init,
      payer = payer,
      space = 8 + 8,
      seeds = [MASTER_SEED],
      bump
   )]
   pub master: Account<'info, Master>,
   #[account(mut)]
   pub payer: Signer<'info>,
   pub system_program: Program<'info, System>,

} 

#[derive(Accounts)]
pub struct CreateBet<'info> {
   #[account(
    init,
    payer = payer,
    space = 8 + 8 + 32 + 8 + 8 + 32 + 8 + 1 + 32 + 8 + 1,
    seeds = [BET_SEED, &(master.last_bet_id + 1).to_le_bytes()],
    bump
   )]
   pub bet: Account<'info, Bet>,
   #[account(mut)]
   pub payer: Signer<'info>,
   #[account(mut, seeds = [MASTER_SEED], bump)]
   pub master: Account<'info, Master>,
   #[account(mut)]
   pub player: Signer<'info>,
   pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnterBet<'info> {
   #[account(
     mut, 
     seeds = [BET_SEED, &bet.id.to_le_bytes()],
     bump,
     constraint = validate_enter_bet(&*bet) @ BetError::CannotEnter
   )]
   pub bet: Account<'info, Bet>,

   #[account(mut)]
   pub player: Signer<'info>,
   pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimBet<'info> {
    #[account(
         mut,
         seeds = [BET_SEED, &bet.id.to_le_bytes()],
         bump,
         constraint  = validate_claim_bet(&bet) @ BetError::CannotClaim
    )]
    pub bet: Account<'info, Bet>,

    /// CHECK 
    #[account(address = bet.pyth_price_key @ BetError::InvalidPythKey)]
    pub pyth: AccountInfo<'info>,

    /// CHECK 
    #[account(mut, address = bet.prediction_a.player)]
    pub player_a:  AccountInfo<'info>,
    
    /// CHECK 
    #[account(mut, address = bet.predidction_b.as_ref().unwrap().player)]
    pub player_b: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}   

 #[derive(Accounts)]
 pub struct CloseBet<'info> {
    #[account(
        mut, 
        seeds = [BET_SEED, &bet.id.to_le_bytes()] /*gets the bet we want to close*/,
        bump,
        close = player , //j ust like init, close, handles the closing of the account
        constraint = validate_close_bet(&bet, player.key()) @ BetError::CannotClose,
    )] 
    pub bet: Account<'info, Bet>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>
 }