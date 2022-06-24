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
use xcvm_protocols::{Swap, SwapError};

// Instantiation of the contract, does nothing.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _: Env,
    _: MessageInfo,
    _: InstantiateMsg,
) -> Result<Response, HackError> {
    deps.storage.set(STATE_KEY, &to_vec(&State {})?);
    Ok(Response::new()
        .add_attribute("unleash", "XCVM")
        .add_attribute("she", "XCVme")
        .add_attribute("he", "XCVyou"))
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
        ExecuteMsg::ETPhoneHome {
            amount_in: Displayed(amount_in),
            amount_out: Displayed(amount_out),
        } => {
            deps.api.debug("ET Phone Home");
            let user_addr = hex::decode(&info.sender.as_bytes()[2..])
                .map_err(|_| HackError::Std(StdError::generic_err("Impossible; QED;")))?;
            let program: Program = (|| {
                Ok(
                    XCVMProgramBuilder::from(Some("ET_Parent".into()), XCVMNetwork::PICASSO)
                        .spawn::<_, SwapError>(
                            Some("ET_Children".into()),
                            XCVMNetwork::ETHEREUM,
                            Vec::new(),
                            XCVMTransfer::from([(XCVMAsset::PICA, ratio_one())]),
                            |child| {
                                Ok(child
                                    .call(Swap::new(
                                        XCVMAsset::PICA,
                                        XCVMAsset::USDC,
                                        amount_in,
                                        amount_out,
                                    ))?
                                    .spawn::<_, SwapError>(
                                        Some("ET_Cousin".into()),
                                        XCVMNetwork::PICASSO,
                                        Vec::new(),
                                        XCVMTransfer::from([
                                            (XCVMAsset::PICA, ratio_one()),
                                            (XCVMAsset::USDC, ratio_one()),
                                        ]),
                                        |child| {
                                            Ok(child.transfer(
                                                user_addr,
                                                XCVMTransfer::from([
                                                    (XCVMAsset::PICA, ratio_one()),
                                                    (XCVMAsset::USDC, ratio_one()),
                                                ]),
                                            ))
                                        },
                                    )?)
                            },
                        )?
                        .build(),
                )
            })()
            .map_err(|_: SwapError| {
                HackError::Std(StdError::generic_err("Couldn't build XCVM program."))
            })?;
            deps.api.debug("ET Program Transmit");
            let actual_balance = deps
                .querier
                .query::<BalanceResponse>(&QueryRequest::Bank(BankQuery::Balance {
                    address: env.contract.address.into(),
                    denom: "1".into(), // heh
                }))?
                .amount
                .amount
                .u128();
            Ok(
                Response::new().add_message(CosmosMsg::Custom(ComposableMsg::XCVM {
                    salt: Vec::new(),
                    funds: XCVMTransfer([(XCVMAsset::PICA, Displayed(actual_balance))].into()),
                    program,
                })),
            )
        }
    }
}

#[entry_point]
pub fn query(_: Deps, _: Env, _: QueryMsg) -> StdResult<QueryResponse> {
    StdResult::Err(StdError::generic_err("Nothing to extract."))
}
