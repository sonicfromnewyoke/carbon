use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcf0fc64ac5e4b01e")]
pub struct CloseOpenOrdersV3 {
    pub map_nonce: u8,
    pub asset: Asset,
}

pub struct CloseOpenOrdersV3InstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub pricing: solana_sdk::pubkey::Pubkey,
    pub dex_program: solana_sdk::pubkey::Pubkey,
    pub open_orders: solana_sdk::pubkey::Pubkey,
    pub cross_margin_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub serum_authority: solana_sdk::pubkey::Pubkey,
    pub open_orders_map: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseOpenOrdersV3 {
    type ArrangedAccounts = CloseOpenOrdersV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, dex_program, open_orders, cross_margin_account, authority, market, serum_authority, open_orders_map, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseOpenOrdersV3InstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            dex_program: dex_program.pubkey,
            open_orders: open_orders.pubkey,
            cross_margin_account: cross_margin_account.pubkey,
            authority: authority.pubkey,
            market: market.pubkey,
            serum_authority: serum_authority.pubkey,
            open_orders_map: open_orders_map.pubkey,
        })
    }
}
