use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe86f73c4ed8f3ecc")]
pub struct PlaceOrderV2 {
    pub price: u64,
    pub size: u64,
    pub side: Side,
    pub order_type: OrderType,
    pub client_order_id: Option<u64>,
}

pub struct PlaceOrderV2InstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub zeta_group: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub dex_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub serum_authority: solana_sdk::pubkey::Pubkey,
    pub greeks: solana_sdk::pubkey::Pubkey,
    pub open_orders: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub market_accounts: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub oracle_backup_feed: solana_sdk::pubkey::Pubkey,
    pub oracle_backup_program: solana_sdk::pubkey::Pubkey,
    pub market_node: solana_sdk::pubkey::Pubkey,
    pub market_mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceOrderV2 {
    type ArrangedAccounts = PlaceOrderV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, margin_account, authority, dex_program, token_program, serum_authority, greeks, open_orders, rent, market_accounts, oracle, oracle_backup_feed, oracle_backup_program, market_node, market_mint, mint_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PlaceOrderV2InstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            margin_account: margin_account.pubkey,
            authority: authority.pubkey,
            dex_program: dex_program.pubkey,
            token_program: token_program.pubkey,
            serum_authority: serum_authority.pubkey,
            greeks: greeks.pubkey,
            open_orders: open_orders.pubkey,
            rent: rent.pubkey,
            market_accounts: market_accounts.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            market_node: market_node.pubkey,
            market_mint: market_mint.pubkey,
            mint_authority: mint_authority.pubkey,
        })
    }
}
