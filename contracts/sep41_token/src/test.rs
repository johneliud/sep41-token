#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::{error::ContractError, our_token::{SibToken, SibTokenClient}};

struct SetUpResult<'a> {
    env: Env,
    client: SibTokenClient<'a>,
    admin: Address,
    sender: Address,
    receiver: Address,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SibToken, ());
    let client = SibTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let receiver = Address::generate(&env);

    client.initialize(&admin, &1_000_000);

    SetUpResult { env, client, admin, sender, receiver }
}

#[test]
fn test_name() {
    let s = setup();
    assert_eq!(s.client.name(), String::from_str(&s.env, "SibToken"));
}

#[test]
fn test_symbol() {
    let s = setup();
    let sym = s.client.symbol();
    assert_eq!(sym, String::from_str(&s.env, "SIB"));
    assert_ne!(sym, String::from_str(&s.env, "Sib"));
}

#[test]
fn test_decimal() {
    let s = setup();
    assert_eq!(s.client.decimals(), 18);
}

#[test]
fn test_transfer() {
    let s = setup();
    s.client.mint(&s.sender, &500);
    assert_eq!(s.client.balance(&s.sender), 500);

    s.client.transfer(&s.sender, &s.receiver, &200);
    assert_eq!(s.client.balance(&s.sender), 300);
    assert_eq!(s.client.balance(&s.receiver), 200);
}

#[test]
fn test_transfer_insufficient_funds() {
    let s = setup();
    s.client.mint(&s.sender, &100);

    let result = s.client.try_transfer(&s.sender, &s.receiver, &500);
    assert_eq!(result, Err(Ok(ContractError::InsufficientFunds)));
}

#[test]
fn test_approve_and_allowance() {
    let s = setup();
    s.client.mint(&s.sender, &1000);

    s.client.approve(&s.sender, &s.receiver, &300, &100);
    assert_eq!(s.client.allowance(&s.sender, &s.receiver), 300);
}

#[test]
fn test_transfer_from() {
    let s = setup();
    let spender = Address::generate(&s.env);

    s.client.mint(&s.sender, &1000);
    s.client.approve(&s.sender, &spender, &400, &100);

    s.client.transfer_from(&spender, &s.sender, &s.receiver, &250);

    assert_eq!(s.client.balance(&s.sender), 750);
    assert_eq!(s.client.balance(&s.receiver), 250);
    assert_eq!(s.client.allowance(&s.sender, &spender), 150);
}

#[test]
fn test_transfer_from_insufficient_allowance() {
    let s = setup();
    let spender = Address::generate(&s.env);

    s.client.mint(&s.sender, &1000);
    s.client.approve(&s.sender, &spender, &100, &100);

    let result = s.client.try_transfer_from(&spender, &s.sender, &s.receiver, &500);
    assert_eq!(result, Err(Ok(ContractError::InsufficientAllowance)));
}

#[test]
fn test_burn() {
    let s = setup();
    s.client.mint(&s.sender, &800);

    s.client.burn(&s.sender, &300);
    assert_eq!(s.client.balance(&s.sender), 500);
}

#[test]
fn test_burn_insufficient_funds() {
    let s = setup();
    s.client.mint(&s.sender, &100);

    let result = s.client.try_burn(&s.sender, &500);
    assert_eq!(result, Err(Ok(ContractError::InsufficientFunds)));
}

#[test]
fn test_burn_from() {
    let s = setup();
    let spender = Address::generate(&s.env);

    s.client.mint(&s.sender, &1000);
    s.client.approve(&s.sender, &spender, &500, &100);

    s.client.burn_from(&spender, &s.sender, &200);

    assert_eq!(s.client.balance(&s.sender), 800);
    assert_eq!(s.client.allowance(&s.sender, &spender), 300);
}

#[test]
fn test_burn_from_insufficient_allowance() {
    let s = setup();
    let spender = Address::generate(&s.env);

    s.client.mint(&s.sender, &1000);
    s.client.approve(&s.sender, &spender, &50, &100);

    let result = s.client.try_burn_from(&spender, &s.sender, &200);
    assert_eq!(result, Err(Ok(ContractError::InsufficientAllowance)));
}

#[test]
fn test_mint() {
    let s = setup();
    s.client.mint(&s.receiver, &750);
    assert_eq!(s.client.balance(&s.receiver), 750);
}

#[test]
fn test_mint_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SibToken, ());
    let client = SibTokenClient::new(&env, &contract_id);

    let receiver = Address::generate(&env);
    let result = client.try_mint(&receiver, &100);
    assert_eq!(result, Err(Ok(ContractError::Unauthorized)));
}
