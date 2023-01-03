use cosmwasm_std::{Addr, Coin, StdResult, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::{
    contract::{execute, instantiate, query},
    msg::{BidResp, ExecuteMsg, InstantiateMsg, QueryMsg},
    ContractError,
};

pub struct BidwasmContract(Addr);

impl BidwasmContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    // Store the code and retrieve the store_code_id
    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    // Perform instantiation for the contract
    #[track_caller]
    pub fn instantiate<'a>(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        owner: impl Into<Option<&'a Addr>>,
        denom: &str,
        description: &str,
        commission: impl Into<Option<u128>>,
    ) -> StdResult<Self> {
        let owner = owner.into();
        let commission = commission.into();

        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                denom: denom.to_string(),
                owner: owner.map(Addr::to_string),
                description: description.to_string(),
                commission,
            },
            &[],
            label,
            None,
        )
        .map(BidwasmContract)
        .map_err(|err| err.downcast().unwrap())
    }

    // Perform bidding to the auction
    #[track_caller]
    pub fn bid(&self, app: &mut App, sender: &Addr, funds: &[Coin]) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecuteMsg::Bid {}, funds)
            .map(|_| ())
            .map_err(|err| err.downcast().unwrap())
    }

    // Closing the auction
    #[track_caller]
    pub fn close(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecuteMsg::Close {}, &[])
            .map(|_| ())
            .map_err(|err| err.downcast().unwrap())
    }

    // Retract the funds
    #[track_caller]
    pub fn retract<'a>(
        &self,
        app: &mut App,
        sender: &Addr,
        recipient: impl Into<Option<&'a Addr>>,
    ) -> Result<(), ContractError> {
        let recipient = recipient.into();

        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::Retract {
                recipient: recipient.map(Addr::to_string),
            },
            &[],
        )
        .map(|_| ())
        .map_err(|err| err.downcast().unwrap())
    }

    pub fn query_total_bid(&self, app: &App, address: &Addr) -> StdResult<Uint128> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::TotalBid {
                address: address.to_string(),
            },
        )
    }

    pub fn query_highest_bid(&self, app: &App) -> StdResult<BidResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::HighestBid {})
    }

    pub fn query_is_closed(&self, app: &App) -> StdResult<bool> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::IsClosed {})
    }

    pub fn query_winner(&self, app: &App) -> StdResult<BidResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Winner {})
    }
}

impl From<BidwasmContract> for Addr {
    fn from(contract: BidwasmContract) -> Self {
        contract.0
    }
}
