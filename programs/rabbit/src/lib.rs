use anchor_lang::prelude::*;

declare_id!("CqBftoLhWzC4PUVCZNGsQ7UwJxfMJHacWqZ9FsxW6YTP");

#[program]
pub mod rabbit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
