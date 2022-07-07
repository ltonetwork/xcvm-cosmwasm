use crate::errors::HackError;
use crate::msg::*;
use crate::state::{State, STATE_KEY};
use cosmwasm_std::{entry_point, to_vec, Addr, DepsMut, Env, MessageInfo, Response, StdError};
use ethabi::{encode, Function, Param, ParamType, StateMutability, Token};
use ethereum_types::H160;
use xcvm_core::*;

fn ratio_one() -> Amount {
    Amount::Ratio(100)
}

// Available protocols
// use xcvm_protocols::{Swap, SwapError};

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
                Param {
                    name: "to".into(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "ownableId".into(),
                    kind: ParamType::String,
                    internal_type: None,
                },
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
                // let contract_address = H160::from_str("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512");
                // .expect("impossible");
                let contract_address =
                    H160::from_slice("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512".as_bytes());

                let encoded_call = Self::ethereum_prototype()
                    .encode_input(&[
                        Token::Address(self.to),
                        Token::String(String::from(self.ownableId.as_str())),
                    ])
                    .map_err(|_| MintError::EncodingFailed)?;
                Ok(encode(&[Token::Address(contract_address), Token::Bytes(encoded_call)]).into())
            }
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
    let addr_string = info.sender.into_string();
    let owner = Addr::unchecked(String::from(addr_string.as_str()));
    let issuer = Addr::unchecked(addr_string);
    let state = State {
        owner,
        issuer,
        max_capacity: msg.max_capacity,
        current_capacity: msg.max_capacity,
    };
    deps.api.debug("Instantiating contract");
    deps.storage.set(STATE_KEY, &to_vec(&state)?);
    Ok(Response::new().add_attribute("issuer", state.issuer.into_string()))
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
        ExecuteMsg::Consume { amount } => {
            deps.api.debug("Consume consume");
            // hex encoded picasso address
            let user_addr = hex::decode(&info.sender.as_bytes()[2..])
                .map_err(|_| HackError::Std(StdError::generic_err("Impossible; QED;")))?;
            let res = Response::new().add_attribute("msg", "consume");
            Ok(res)
        }
        ExecuteMsg::Mint { owner } => {
            deps.api.debug("Minting");
            // hex encoded picasso address
            let user_addr = hex::decode(&info.sender.as_bytes()[2..])
                .map_err(|_| HackError::Std(StdError::generic_err("Impossible; QED;")))?;
            // let owner_eth_addr = H160::from_str(&owner);
            let owner_eth_addr = H160::from_slice(owner.as_bytes());

            let program: Program = (|| {
                Ok(
                    XCVMProgramBuilder::from(Some("Mint_parent".into()), XCVMNetwork::PICASSO)
                        .spawn::<_, MintError>(
                            Some("Mint_children".into()),
                            XCVMNetwork::ETHEREUM,
                            Vec::new(),
                            XCVMTransfer::empty(),
                            |f| Ok(f.call(Mint::new(owner_eth_addr, "ownable-1".to_string()))?),
                        )?
                        .build(),
                )
            })()
            .map_err(|_: MintError| {
                HackError::Std(StdError::generic_err("Couldn't build XCVM program."))
            })?;

            let res = Response::new().add_attribute("msg", "mint");
            Ok(res)
        }
        ExecuteMsg::Transfer { to } => {
            deps.api.debug("Transferring");
            let user_addr = hex::decode(&info.sender.as_bytes()[2..])
                .map_err(|_| HackError::Std(StdError::generic_err("Impossible; QED;")))?;
            // let program: Program = (|| {})().map_err(|_: SwapError| {
            //     HackError::Std(StdError::generic_err("Couldn't build XCVM program."))
            // })?;

            let res = Response::new().add_attribute("msg", "transfer");
            Ok(res)
        }
    }
}
