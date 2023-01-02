use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can perform this action")]
    Unauthorized { owner: String },

    #[error("Invalid bid: Contract owner {owner} cannot perform this action")]
    InvalidAction { owner: String },

    #[error("Auction is already closed")]
    ClosedAcution,

    #[error(
        "Invalid bid: Amount of token sent ({funds}) are lower than commission ({commission})"
    )]
    InsufficientFundsForCommission { funds: Uint128, commission: Uint128 },

    #[error("Invalid bid: Amount of token sent cannot be 0")]
    InsufficientFunds,

    #[error("Invalid bid: Proposed bid (existing + new bid = {existing} + {funds} = {new_bid}) is lower than actual maximum bid ({max_bid})")]
    InsufficientBid {
        existing: Uint128,
        funds: Uint128,
        new_bid: Uint128,
        max_bid: Uint128,
    },
}
