use anchor_lang::prelude::*;

#[account]

pub struct Todo {
    pub content: String,
    pub completed: bool,
}
impl Profile{
    const SPACE: usize = (4 + 100) + 32 + 32;
}

#[account]

pub struct Profile {
    pub name: String, // max 100
    pub key: Pubkey,
    pub authority: Pubkey
}