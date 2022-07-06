use crate::errors::HackError;
use crate::msg::{ComposableMsg, ExecuteMsg, InstantiateMsg, Program, QueryMsg};
use crate::state::{State, STATE_KEY};
use cosmwasm_std::{
    entry_point, to_vec, BalanceResponse, BankQuery, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, QueryResponse, Response, StdError, StdResult, Addr,
};
use xcvm_core::*;
use ethabi::{encode, ethereum_types::H160, Function, Param, ParamType, StateMutability, Token};

fn ratio_one() -> Amount {
    Amount::Ratio(100)
}

// Available protocols
use xcvm_protocols::{Swap, SwapError};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mint {
    to: H160,
    ownableId: String,
}

impl Mint {
    pub fn new(to: H160, ownableId: String) -> Self {
        Mint { to, ownableId }
    }

    pub fn ethereum_prototype() -> Function {
        Function {
            name: "mint".to_owned(),
            inputs: vec![
                Param { name: "to".into(), kind: ParamType::Address, internal_type: None },
                Param { name: "ownableId".into(), kind: ParamType::String, internal_type: None },
            ],
            outputs: vec![],
            constant: None,
            state_mutability: StateMutability::Payable,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MintError {
    UnsupportedNetwork,
    EncodingFailed,
}

impl XCVMProtocol<XCVMNetwork> for Mint {
    type Error = MintError;
    fn serialize(&self, network: XCVMNetwork) -> Result<Vec<u8>, Self::Error> {
        match network {
            XCVMNetwork::ETHEREUM => {
                let contract_address =
                    H160::from_str("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512")
                        .expect("impossible");
                let encoded_call = Self::ethereum_prototype()
                    .encode_input(&[Token::Address(self.to), Token::String(self.ownableId)])
                    .map_err(|_| MintError::EncodingFailed)?;
                Ok(encode(&[Token::Address(contract_address), Token::Bytes(encoded_call)]).into())
            },
            _ => Err(MintError::UnsupportedNetwork),
        }
    }
}



// Instantiation of the contract, does nothing.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, HackError> {
    let state = State {
        owner: info.sender,
        issuer: info.sender,
        max_capacity: msg.max_capacity,
        current_capacity: msg.max_capacity,
    };
    deps.api.debug("Instantiating contract");
    deps.storage.set(STATE_KEY, &to_vec(&state)?);
    Ok(Response::new()
        .add_attribute("max_capacity", msg.max_capacity)
        .add_attribute("issuer", info.sender)
    )
}

// Actual execution of the contract.
// Spawn a sub program to ETH, does a swap and spawn back to Picasso.
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ComposableMsg>, HackError> {
    match msg {
        ExecuteMsg::Consume {
            amount: msg.amount,
        } => {
            deps.api.debug("Consume consume");
            // hex encoded picasso address
            let user_addr = hex::decode(&info.sender.as_bytes()[2..])
            .map_err(|_| HackError::Std(StdError::generic_err("Impossible; QED;")))?;
            let program: Program = (|| {
                Ok()
            })()
        }
        ExecuteMsg::Mint {
            owner: Displayed(msg.owner.into()),
        } => {
            deps.api.debug("Minting");
            // hex encoded picasso address
            let user_addr = hex::decode(&info.sender.as_bytes()[2..])
            .map_err(|_| HackError::Std(StdError::generic_err("Impossible; QED;")))?;
            let owner_eth_addr =  H160::from_str(&owner);
            let program: Program = (|| {
                Ok(
                    XCVMProgramBuilder::from(Some("Mint_parent".into()), XCVMNetwork::PICASSO)
                        .spawn::<_, MintError>(
                            Some("Mint_children".into()),
                            XCVMNetwork::ETHEREUM,
                            Vec::new(),
                            (),
                            |f| {
                                Ok(f.call(Mint::new(
                                        owner_eth_addr,
                                        "ownable-1"
                                    ))?
                                )
                            },
                        )?
                        .build(),
                )
            })()
            .map_err(|_: MintError| {
                HackError::Std(StdError::generic_err("Couldn't build XCVM program."))
            })?;
        }
        ExecuteMsg::Transfer {
            to: msg.to,
        } => {
            deps.api.debug("Transferring");
            let user_addr = hex::decode(&info.sender.as_bytes()[2..])
            .map_err(|_| HackError::Std(StdError::generic_err("Impossible; QED;")))?;
            let program: Program = (|| {
                Ok()
            })()
            .map_err(|_: MintError| {
                HackError::Std(StdError::generic_err("Couldn't build XCVM program."))
            })?;
        }
    }
}

#[entry_point]
pub fn query(_: Deps, _: Env, _: QueryMsg) -> StdResult<QueryResponse> {
    StdResult::Err(StdError::generic_err("Nothing to extract."))
}

