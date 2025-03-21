use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum UpdateAuthority {
    None,
    Address(solana_sdk::pubkey::Pubkey),
    Collection(solana_sdk::pubkey::Pubkey),
}
