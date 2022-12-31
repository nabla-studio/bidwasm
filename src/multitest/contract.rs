use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper};

use crate::contract::{execute, instantiate, query};

pub struct BidwasmContract(Addr);

impl BidwasmContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }
}

impl From<BidwasmContract> for Addr {
    fn from(contract: BidwasmContract) -> Self {
        contract.0
    }
}
