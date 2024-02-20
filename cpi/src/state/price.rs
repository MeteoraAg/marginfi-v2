use anchor_lang::prelude::*;

#[repr(u8)]
#[cfg_attr(any(feature = "test", feature = "client"), derive(PartialEq, Eq))]
#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum OracleSetup {
    None,
    PythEma,
    SwitchboardV2,
}

#[derive(Copy, Clone, Debug)]
pub enum PriceBias {
    Low,
    High,
}
