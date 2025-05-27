use anchor_lang::prelude::*;
mode state;

declare_id!("HwqZq2QTWoX7ep4zfSYqgjhuAKnEjavvzQRFHmRF3gZA");
#[program]
pub mod solana {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, name: String) -> Result<()> {

        let key = ctx.accounts.creator.key();
        let profile = &ctx.accounts.profile;
        profile.name = name;
        profile.authority = ctx.accounts.creator.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Profile::SPACE,
        // create a seeds array to ensure the profile is unique with public key
        seeds = [b"profile", creator.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    pub system_program: Program<'info, System>,
}
