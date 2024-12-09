use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

declare_id!("31AsjBvWTL93SSjVEHM8p3JqvuwiRuTh8P9grM7Tzs8L");

#[program]
pub mod custom_nft_minter {
    use token::spl_token;

    use super::*;
    pub fn mint_custom_nft(
        ctx: Context<MintCustomNFT>,
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
    ) -> Result<()> {
        // Initialize the mint (SPL Token Program)
        token::initialize_mint(
            ctx.accounts.initialize_mint_ctx(),
            0, // Decimals for NFT
            &ctx.accounts.authority.key(),
            Some(&ctx.accounts.authority.key()),
        )?;

        // Create ATA for the user - Account that can hold a specific type of token - For the user to hold the NFT
        anchor_spl::associated_token::create(ctx.accounts.create_ata_ctx())?;

        // Mint 1 NFT to the ATA
        token::mint_to(ctx.accounts.mint_to_ctx(), 1)?;

        // Set mint authority to None (immutable) - No one can mint more of this NFTs
        token::set_authority(
            ctx.accounts.set_authority_ctx(),
            spl_token::instruction::AuthorityType::MintTokens,
            None,
        )?;

        // Create Metadata Account via CPI
        // let metadata_seeds = &[
        //     b"metadata",
        //     ctx.accounts.token_metadata_program.key().as_ref(),
        //     ctx.accounts.mint.key().as_ref(),
        //     &[ctx.bumps.get("metadata").unwrap()],
        // ];
        // let create_metadata_ix = CreateMetadataAccountV3 {
        //     // Fill in the necessary fields for metadata creation
        //     // This is a placeholder; you need to populate it based on your requirements
        //     // ...
        // };
        // Invoke the metadata creation instruction here
        // ...

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCustomNFT<'info> {
    /// The authority who is minting the NFT
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This is the metadata account derived from the mint and token metadata program
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        bump
    )]
    pub metadata: UncheckedAccount<'info>,

    /// The mint account for the NFT, initialized with 0 decimals and authority set to the signer
    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority.key(),
        mint::freeze_authority = authority.key()
    )]
    pub mint: Account<'info, Mint>,

    /// The associated token account for holding the NFT, initialized for the signer
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// The Metaplex Token Metadata Program
    #[account(address = Pubkey::new_from_array(mpl_token_metadata::ID.to_bytes()))]
    /// CHECK: This is the Metaplex Token Metadata Program
    pub token_metadata_program: UncheckedAccount<'info>,

    /// Standard system program
    pub system_program: Program<'info, System>,

    /// SPL Token program
    pub token_program: Program<'info, Token>,

    /// Associated Token program
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,

    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> MintCustomNFT<'info> {
    /// Context for initializing the mint
    fn initialize_mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::InitializeMint<'info>> {
        let cpi_accounts = token::InitializeMint {
            mint: self.mint.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    /// Context for creating the associated token account
    fn create_ata_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, anchor_spl::associated_token::Create<'info>> {
        let cpi_accounts = anchor_spl::associated_token::Create {
            payer: self.authority.to_account_info(),
            associated_token: self.token_account.to_account_info(),
            authority: self.authority.to_account_info(),
            mint: self.mint.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            // rent: self.rent.to_account_info(), // Not needed as it's implicitly handled
        };
        CpiContext::new(
            self.associated_token_program.to_account_info(),
            cpi_accounts,
        )
    }

    /// Context for minting tokens
    fn mint_to_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_accounts = token::MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    /// Context for setting authority
    fn set_authority_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::SetAuthority<'info>> {
        let cpi_accounts = token::SetAuthority {
            current_authority: self.authority.to_account_info(),
            account_or_mint: self.mint.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
