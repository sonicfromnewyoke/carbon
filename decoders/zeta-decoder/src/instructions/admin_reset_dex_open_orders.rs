use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x73c65a11d28bc1ee")]
pub struct AdminResetDexOpenOrders {
    pub asset: Asset,
}

pub struct AdminResetDexOpenOrdersInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub dex_program: solana_sdk::pubkey::Pubkey,
    pub open_orders: solana_sdk::pubkey::Pubkey,
    pub cross_margin_account: solana_sdk::pubkey::Pubkey,
    pub pricing_admin: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub serum_authority: solana_sdk::pubkey::Pubkey,
    pub event_queue: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminResetDexOpenOrders {
    type ArrangedAccounts = AdminResetDexOpenOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, dex_program, open_orders, cross_margin_account, pricing_admin, authority, market, bids, asks, serum_authority, event_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AdminResetDexOpenOrdersInstructionAccounts {
            state: state.pubkey,
            dex_program: dex_program.pubkey,
            open_orders: open_orders.pubkey,
            cross_margin_account: cross_margin_account.pubkey,
            pricing_admin: pricing_admin.pubkey,
            authority: authority.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            serum_authority: serum_authority.pubkey,
            event_queue: event_queue.pubkey,
        })
    }
}
