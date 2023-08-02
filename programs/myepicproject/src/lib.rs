use anchor_lang::prelude::*;

declare_id!("9uJeg9wfWjti7chiHsJS7M289JsyxtjHCpAGFGf4amRt");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff {}