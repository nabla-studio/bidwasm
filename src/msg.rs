use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

// Instantiate message contains information about the auction itself:
// - denom for the token bids
// - owner address for the auction management
// - description for the auction
#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
    pub owner: Option<String>,
    pub description: String,
    pub commission: Option<u128>,
}

// Executing the actions in the smart contract
#[cw_serde]
pub enum ExecuteMsg {
    Bid {},
    Close {},
    Retract { recipient: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u128)]
    TotalBid { address: String },
    #[returns(BidResp)]
    HighestBid {},
    #[returns(bool)]
    IsClosed {},
    #[returns(BidResp)]
    Winner {},
}

#[cw_serde]
pub struct BidResp {
    pub address: Addr,
    pub amount: Uint128,
}
