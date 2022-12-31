#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, State, Status, CONFIG, STATE};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:bidwasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Define the owner address for the contract
    let owner;

    // Current state for the auction is an "open" status and no bid
    STATE.save(
        deps.storage,
        &State {
            current_status: Status::Open,
            highest_bid: None,
        },
    )?;

    // If the owner is not passed to the instantiate function, use the sender
    // address as the owner one
    match msg.owner {
        Some(str_owner) => owner = deps.api.addr_validate(&str_owner)?,
        None => owner = info.sender,
    }

    // The configuration for the auction corresponds to:
    //  - passed denom for the bid tokens;
    //  - the owner address for the auction, which is able to close the auction
    //  but cannot partecipate in bid requests;
    //  - the description of the auction.
    CONFIG.save(
        deps.storage,
        &Config {
            denom: msg.denom,
            owner: owner,
            description: msg.description,
        },
    )?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
