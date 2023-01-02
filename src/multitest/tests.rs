use cosmwasm_std::{coins, Addr, Uint128};
use cw_multi_test::App;

use crate::{
    state::{Config, State, Status, BIDS, CONFIG, STATE},
    ContractError,
};

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

// START --> Bidding Tests

#[test]
fn owner_cannot_bid() {
    // Define participant
    let owner = Addr::unchecked("owner");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &owner, coins(10_000_000, UATOM))
            .unwrap();
    });

    let code_id = BidwasmContract::store_code(&mut app);

    // Instantiate contract with owner different than sender
    let contract = BidwasmContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidwasm contract",
        &owner,
        UATOM,
        "Supercomputer #2207 bidding",
        500_000,
    )
    .unwrap();

    // Try making the owner bidding to his own auction
    let err = contract
        .bid(&mut app, &owner, &coins(10_000_000, UATOM))
        .unwrap_err();

    // Verify that the contract fails if the contract owner bid to his own
    // auction
    assert_eq!(
        ContractError::InvalidAction {
            owner: owner.to_string()
        },
        err
    );

    // No funds should be moved
    assert_eq!(
        app.wrap().query_all_balances(owner).unwrap(),
        coins(10_000_000, UATOM)
    );

    assert_eq!(app.wrap().query_all_balances(contract.addr()).unwrap(), &[]);
}

#[test]
fn insufficient_funds_bid() {
    // Define participant
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10_000_000, UATOM))
            .unwrap();
    });

    let code_id = BidwasmContract::store_code(&mut app);

    // Instantiate contract with owner different than sender
    let contract = BidwasmContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidwasm contract",
        &owner,
        UATOM,
        "Supercomputer #2207 bidding",
        500_000,
    )
    .unwrap();

    // Try making the owner bidding to his own auction
    let err = contract
        .bid(&mut app, &sender, &coins(100_000, UATOM))
        .unwrap_err();

    // Verify that the contract fails if the sender sends lower funds than
    // required for the commission by the auction
    assert_eq!(
        ContractError::InsufficientFundsForCommission {
            funds: Uint128::new(100_000),
            commission: Uint128::new(500_000)
        },
        err
    );

    // No funds should be moved
    assert_eq!(
        app.wrap().query_all_balances(sender).unwrap(),
        coins(10_000_000, UATOM)
    );

    assert_eq!(app.wrap().query_all_balances(owner).unwrap(), &[]);

    assert_eq!(app.wrap().query_all_balances(contract.addr()).unwrap(), &[]);
}

#[test]
fn simple_bid_no_commission() {
    // Define participant
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(3_500_000, UATOM))
            .unwrap();
    });

    let code_id = BidwasmContract::store_code(&mut app);

    // Instantiate contract with owner different than sender
    let contract = BidwasmContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidwasm contract",
        &owner,
        UATOM,
        "Supercomputer #2207 bidding",
        0,
    )
    .unwrap();

    // Making a simple bid
    contract
        .bid(&mut app, &sender, &coins(3_500_000, UATOM))
        .unwrap();

    // let bids = BIDS.query(&app.wrap(), contract.addr().clone()).unwrap();
    let bid = BIDS
        .query(&app.wrap(), contract.addr().clone(), &sender)
        .unwrap();

    // Check if bid is stored in the state
    assert_eq!(bid, Some(Uint128::new(3_500_000)));

    // sender should have not any balance
    assert_eq!(app.wrap().query_all_balances(sender).unwrap(), &[]);

    // owner should have not any balance
    assert_eq!(app.wrap().query_all_balances(owner).unwrap(), &[]);

    // contract should store the whole bid (no commission)
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(3_500_000, UATOM)
    );
}

#[test]
fn simple_bid() {
    // Define participant
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(3_500_000, UATOM))
            .unwrap();
    });

    let code_id = BidwasmContract::store_code(&mut app);

    // Instantiate contract with owner different than sender
    let contract = BidwasmContract::instantiate(
        &mut app,
        code_id,
        &owner,
        "Bidwasm contract",
        &owner,
        UATOM,
        "Supercomputer #2207 bidding",
        500_000,
    )
    .unwrap();

    // Making a simple bid
    contract
        .bid(&mut app, &sender, &coins(3_500_000, UATOM))
        .unwrap();

    // let bids = BIDS.query(&app.wrap(), contract.addr().clone()).unwrap();
    let bid = BIDS
        .query(&app.wrap(), contract.addr().clone(), &sender)
        .unwrap();

    // Check if bid is stored in the state
    assert_eq!(bid, Some(Uint128::new(3_000_000)));

    // sender should have not any balance
    assert_eq!(app.wrap().query_all_balances(sender).unwrap(), &[]);

    // owner should have got the commission
    assert_eq!(
        app.wrap().query_all_balances(owner).unwrap(),
        coins(500_000, UATOM)
    );

    // contract should store bid minus commission
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(3_000_000, UATOM)
    );
}

// END --> Bidding Tests
