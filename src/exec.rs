use cosmwasm_std::{coins, BankMsg, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};

use crate::{
    state::{Status, BIDS, CONFIG, STATE},
    ContractError,
};

pub fn bid(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let owner = CONFIG.load(deps.storage)?.owner;
    let commission = Uint128::new(CONFIG.load(deps.storage)?.commission);
    let denom = CONFIG.load(deps.storage)?.denom;

    let mut state = STATE.load(deps.storage)?;
    let highest_bid = state.highest_bid;

    let mut resp = Response::new();

    // If auction is already closed, then bid cannot be processed
    if state.current_status == Status::Closed {
        return Err(ContractError::ClosedAcution);
    }

    // Owner of the auction cannot bid
    if owner == info.sender {
        return Err(ContractError::InvalidAction {
            owner: owner.to_string(),
        });
    }

    // Retrieve the highest bid or get a default value
    let highest_bid_amount = match highest_bid {
        Some(highest_bid) => highest_bid.1,
        None => Uint128::new(0),
    };

    // Retrieve funds or provide a default value
    let funds = match info.funds.iter().find(|coin| coin.denom == denom) {
        Some(funds) => funds.amount,
        None => return Err(ContractError::InsufficientFunds),
    };

    // If sender needs to pay a commission and the provided funds are less than
    // the required amount
    if !commission.is_zero() && funds < commission {
        // Return error for insufficient funds
        return Err(ContractError::InsufficientFundsForCommission { funds, commission });
    }

    // Calculate the bid without commissions
    let net_bid = funds - commission;

    // If the sender bid is greater than the current maximum bid
    let existing_bid = match BIDS.may_load(deps.storage, &info.sender)? {
        Some(existing_bid) => existing_bid,
        None => Uint128::new(0),
    };

    // If the total bid of the user is less or the same as the highest,
    // bidding should fail
    let new_bid = net_bid + existing_bid;
    if new_bid <= highest_bid_amount {
        return Err(ContractError::InsufficientBid {
            existing: existing_bid,
            funds,
            new_bid: net_bid,
            max_bid: highest_bid_amount,
        });
    }

    // Otherwise we should process the bid
    state.highest_bid = Some((info.sender.clone(), new_bid));

    // If there is any commission, we should send them to the contract owner
    if !commission.is_zero() {
        let funds: Vec<_> = coins(commission.u128(), denom);

        // Create a bank message to send funds to the contract owner
        let commission_msg = BankMsg::Send {
            to_address: owner.into_string(),
            amount: funds,
        };

        resp = resp
            .add_message(commission_msg)
            .add_attribute("commission_payer", info.sender.as_str());
    }

    // Update the bid for the sender
    BIDS.update(deps.storage, &info.sender, |_| -> StdResult<_> {
        Ok(new_bid)
    })?;

    // Update the state for the auction
    STATE.save(deps.storage, &state)?;

    resp = resp
        .add_attribute("action", "bid")
        .add_attribute("sender", info.sender.as_str())
        .add_attribute("current_highest_bid", new_bid);

    Ok(resp)
}
