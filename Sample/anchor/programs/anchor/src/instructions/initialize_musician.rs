use anchor_lang::prelude::*;

use crate::states::{Musician, Style};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeMusician<'info> {
    #[account(init, payer = signer, seeds = [b"musician", signer.key().as_ref()], bump, space = 8 + 50 + 11 + 4)]
    pub musician: Account<'info, Musician>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeMusician<'info> {
    pub fn initialize_musician(&mut self, name: String, style: Style) -> Result<()> {
        let musician = &mut self.musician;
        musician.name = name;
        musician.style = style;
        musician.music_amount = 0;
        Ok(())
    }
}
