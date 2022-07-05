use crate::errors::HackError;
use crate::msg::{ComposableMsg, ExecuteMsg, InstantiateMsg, Program, QueryMsg};
use crate::state::{State, STATE_KEY};
use cosmwasm_std::{
    entry_point, to_vec, BalanceResponse, BankQuery, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, QueryResponse, Response, StdError, StdResult,
};
use xcvm_core::*;

fn ratio_one() -> Amount {
    Amount::Ratio(100)
}

// Available protocols
use xcvm_protocols::{Swap, SwapError, Mint, MintError};

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
        },
        ExecuteMsg::Transfer {
            to: msg.to,
        } => {
            deps.api.debug("Minting");
            // hex encoded picasso address
            let user_addr = hex::decode(&info.sender.as_bytes()[2..])
            .map_err(|_| HackError::Std(StdError::generic_err("Impossible; QED;")))?;
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
                                        to,
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
        },
    }
}

#[entry_point]
pub fn query(_: Deps, _: Env, _: QueryMsg) -> StdResult<QueryResponse> {
    StdResult::Err(StdError::generic_err("Nothing to extract."))
}

