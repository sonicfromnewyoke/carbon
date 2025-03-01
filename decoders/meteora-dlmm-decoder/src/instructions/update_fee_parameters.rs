use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8080d05bf6351fb0")]
pub struct UpdateFeeParameters {
    pub fee_parameter: FeeParameter,
}

pub struct UpdateFeeParametersInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFeeParameters {
    type ArrangedAccounts = UpdateFeeParametersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, admin, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateFeeParametersInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
