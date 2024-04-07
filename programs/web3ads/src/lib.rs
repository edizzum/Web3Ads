use anchor_lang::prelude::;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

declare_id!("Fg6PaFzntrJZ1Ym1FzKSAkJS2LpkuSqS1fyyKN1LL5y");

#[program]
pub mod web3ads {
    use super::;

    pub fn mint_nft(ctx: Context<MintNft>, url: String) -> Result<()> {
        let nft_info = &mut ctx.accounts.nft_info;
        nft_info.url = url;
        nft_info.owner = *ctx.accounts.minter.key;
        nft_info.claimable_tokens = 0;
        Ok(())
    }

    pub fn increase_claimable_tokens(ctx: Context<AccessNft>) -> Result<()> {
        let nft_info = &mut ctx.accounts.nft_info;
        nft_info.claimable_tokens += 5; 
        Ok(())
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        let nft_info = &mut ctx.accounts.nft_info;
        let amount = nft_info.claimable_tokens;

        // Token transfer 
        let ix = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        token::transfer(ctx.accounts.into_transfer_context(), amount)?;

        // updating claim amount
        nft_info.claimable_tokens = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(init, payer = minter, space = 8 + 256 + 32 + 8)]
    pub nft_info: Account<'info, NftInfo>,
    #[account(mut)]
    pub minter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AccessNft<'info> {
    #[account(mut)]
    pub nft_info: Account<'info, NftInfo>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub nft_info: Account<'info, NftInfo>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
}

#[account]
pub struct NftInfo {
    pub url: String,
    pub owner: Pubkey,
    pub claimable_tokens: u64,
}