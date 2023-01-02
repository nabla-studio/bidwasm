use cosmwasm_std::Addr;
use cw_multi_test::App;

use crate::state::{Config, State, Status, CONFIG, STATE};

use super::contract::BidwasmContract;

const UATOM: &str = "uatom";

// START --> Auction Opening Tests

#[test]
fn open_auction_with_owner() {
    // Define participants
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::default();

    let code_id = BidwasmContract::store_code(&mut app);

    // Instantiate contract with owner different than sender
    let contract = BidwasmContract::instantiate(
        &mut app,
        code_id,
        &sender,
        "Bidwasm contract",
        &owner,
        UATOM,
        "Supercomputer #2207 bidding",
        500_000,
    )
    .unwrap();

    // Query the contract state
    let state = STATE.query(&app.wrap(), contract.addr().clone()).unwrap();

    // Verify that contract state is correct
    assert_eq!(
        state,
        State {
            current_status: Status::Open,
            highest_bid: None,
        }
    );

    // Query the contract configuration
    let config = CONFIG.query(&app.wrap(), contract.addr().clone()).unwrap();

    // Verify that contract configuration is correct
    assert_eq!(
        config,
        Config {
            denom: UATOM.to_string(),
            owner,
            description: "Supercomputer #2207 bidding".to_string(),
            commission: 500_000
        }
    );
}

#[test]
fn open_auction_without_owner() {
    // Define participant
    let owner = Addr::unchecked("owner");

    let mut app = App::default();

    let code_id = BidwasmContract::store_code(&mut app);

    // Instantiate contract without expressing the owner
    let contract = BidwasmContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidwasm contract",
        None,
        UATOM,
        "Supercomputer #2207 bidding",
        500_000,
    )
    .unwrap();

    // Query the contract state
    let state = STATE.query(&app.wrap(), contract.addr().clone()).unwrap();

    // Verify that contract state is correct
    assert_eq!(
        state,
        State {
            current_status: Status::Open,
            highest_bid: None,
        }
    );

    // Query the contract configuration
    let config = CONFIG.query(&app.wrap(), contract.addr().clone()).unwrap();

    // Verify that contract configuration is correct (if no owner is provided,
    // default owner is the contract creator).
    assert_eq!(
        config,
        Config {
            denom: UATOM.to_string(),
            owner,
            description: "Supercomputer #2207 bidding".to_string(),
            commission: 500_000
        }
    );
}

#[test]
fn open_auction_without_commission() {
    // Define participant
    let owner = Addr::unchecked("owner");

    let mut app = App::default();

    let code_id = BidwasmContract::store_code(&mut app);

    // Instantiate contract without expressing the commission
    let contract = BidwasmContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidwasm contract",
        &owner,
        UATOM,
        "Supercomputer #2207 bidding",
        None,
    )
    .unwrap();

    // Query the contract state
    let state = STATE.query(&app.wrap(), contract.addr().clone()).unwrap();

    // Verify that contract state is correct
    assert_eq!(
        state,
        State {
            current_status: Status::Open,
            highest_bid: None,
        }
    );

    // Query the contract configuration
    let config = CONFIG.query(&app.wrap(), contract.addr().clone()).unwrap();

    // Verify that contract configuration is correct (if no owner is provided,
    // default owner is the contract creator).
    assert_eq!(
        config,
        Config {
            denom: UATOM.to_string(),
            owner,
            description: "Supercomputer #2207 bidding".to_string(),
            commission: 0
        }
    );
}

// END --> Auction Opening Tests
