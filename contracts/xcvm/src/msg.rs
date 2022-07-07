use cosmwasm_std::{Addr, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use xcvm_core::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub max_capacity: u8,
    pub ownable_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // consumes percentage of remaining potion
    #[serde(rename = "consume")]
    Consume { amount: u8 },
    // transfers ownership
    #[serde(rename = "transfer")]
    Transfer { to: Addr },
    // mints on eth side
    #[serde(rename = "mint")]
    Mint { owner: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCurrentAmount {},
    GetOwner {},
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ComposableMsg {
    XCVM {
        salt: Vec<u8>,
        funds: XCVMTransfer<Displayed<u128>>,
        program:
            XCVMProgram<VecDeque<XCVMInstruction<XCVMNetwork, Vec<u8>, Vec<u8>, XCVMTransfer>>>,
    },
}

impl CustomMsg for ComposableMsg {}

pub type Program =
    XCVMProgram<VecDeque<XCVMInstruction<XCVMNetwork, Vec<u8>, Vec<u8>, XCVMTransfer>>>;
