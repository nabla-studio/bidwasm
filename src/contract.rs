#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, State, Status, CONFIG, STATE};
use crate::{exec, query};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // If commission is passed as an argument, use it. Otherwise, use 0
    let commission = msg.commission.unwrap_or_default();

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
    let owner = match msg.owner {
        Some(str_owner) => deps.api.addr_validate(&str_owner)?,
        None => info.sender,
    };

    // The configuration for the auction corresponds to:
    //  - passed denom for the bid tokens;
    //  - the owner address for the auction, which is able to close the auction
    //  but cannot partecipate in bid requests;
    //  - the description of the auction.
    CONFIG.save(
        deps.storage,
        &Config {
            denom: msg.denom,
            owner,
            description: msg.description,
            commission,
        },
    )?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::{Bid, Close, Retract};

    match msg {
        Bid {} => exec::bid(deps, info),
        Close {} => exec::close(deps, info),
        Retract { recipient } => exec::retract(deps, info, recipient),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TotalBid { address } => to_binary(&query::total_bid(deps, address)?),
        QueryMsg::HighestBid {} => to_binary(&query::highest_bid(deps)?),
        QueryMsg::IsClosed {} => to_binary(&query::is_closed(deps)?),
        QueryMsg::Winner {} => to_binary(&query::winner(deps)?),
    }
}
