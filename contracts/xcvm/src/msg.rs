use cosmwasm_std::CustomMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use xcvm_core::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    #[serde(rename = "et_phone_home")]
    ETPhoneHome { amount_in: Displayed<u128>, amount_out: Displayed<u128> }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

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
