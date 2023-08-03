use anchor_lang::prelude::*;

declare_id!("9uJeg9wfWjti7chiHsJS7M289JsyxtjHCpAGFGf4amRt");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

	// Ahora la funcion recibe el gif_link como parametro y se referencia al usuario desde el contexto
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
    };
		
	// Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

}


// Se agregan variables al contexto de StartStuffOff.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Se especifican los datos que se quieren agregar en AddGifs
// Se agrega el Signer quien es el que llama al AddGifs metodo para guardarlo
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,

}

// Se crea una estructura custom para trabajar
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Aca se define que se quiere guardar en la cuenta
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Se agrega el vector de tipo ItemStruct a la cuenta
    pub gif_list: Vec<ItemStruct>,
}