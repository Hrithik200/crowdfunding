use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod crowdfunding {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String) -> Result<()> {
        let campagin = &mut ctx.accounts.campagin;
        campagin.name = name;
        campagin.description = description;
        campagin.amount_donated = 0;
        campagin.admin=*ctx.accounts.user.key; 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user,space=9000, seeds=[b"CAMPAIGN_DEMO".as_ref(),user.key().as_ref()],bump)]
    pub campagin: Account<'info, Campagin>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]

pub struct Campagin {
    pub name: String,
    pub description: String,
    pub amount_donated: u64,
    pub admin:Pubkey
    
}
