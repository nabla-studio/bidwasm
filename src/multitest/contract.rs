use cosmwasm_std::{Addr, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::{
    contract::{execute, instantiate, query},
    msg::InstantiateMsg,
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
    ) -> StdResult<Self> {
        let owner = owner.into();

        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                denom: denom.to_string(),
                owner: owner.map(Addr::to_string),
                description: description.to_string(),
            },
            &[],
            label,
            None,
        )
        .map(BidwasmContract)
        .map_err(|err| err.downcast().unwrap())
    }
}

impl From<BidwasmContract> for Addr {
    fn from(contract: BidwasmContract) -> Self {
        contract.0
    }
}
