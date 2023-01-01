use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

// Auction possible status
#[cw_serde]
pub enum Status {
    Open,
    Closed,
}

// Auction Configuration information
#[cw_serde]
pub struct Config {
    /// denom of the token to bid
    pub denom: String,
    // owner of the auction
    pub owner: Addr,
    // auction description
    pub description: String,
    // commission on each valid bid
    pub commission: u64,
}

// Auction Current status
#[cw_serde]
pub struct State {
    // status of the auction (open or closed)
    pub current_status: Status,
    // current highest bid to the auction
    pub highest_bid: Option<(Addr, Uint128)>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
pub const BIDS: Map<&Addr, Uint128> = Map::new("bids");
