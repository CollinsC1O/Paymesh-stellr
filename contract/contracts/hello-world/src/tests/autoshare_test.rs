#![cfg(test)]
use crate::{AutoShareContract, AutoShareContractClient};
use crate::base::types::AutoShareDetails;
use crate::base::errors::Error;
use soroban_sdk::{testutils::{Address as _, Events}, Address, BytesN, Env, String};

#[test]
fn test_create_and_get_success() {
    let env = Env::default();
    let contract_id = env.register(AutoShareContract, ());
    let client = AutoShareContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let id = BytesN::from_array(&env, &[1u8; 32]);
    let name = String::from_str(&env, "Platform Split");

    client.create(&id, &name, &creator);

    let result = client.get(&id);
    assert_eq!(result.name, name);
    assert_eq!(result.creator, creator);
}

#[test]
#[should_panic] // Should fail with AlreadyExists error
fn test_duplicate_id_fails() {
    let env = Env::default();
    let contract_id = env.register(AutoShareContract, ());
    let client = AutoShareContractClient::new(&env, &contract_id);

    let id = BytesN::from_array(&env, &[1u8; 32]);
    let name = String::from_str(&env, "Test");
    let creator = Address::generate(&env);

    client.create(&id, &name, &creator);
    client.create(&id, &name, &creator); // Duplicate call
}

#[test]
#[should_panic] // Should fail with NotFound error
fn test_get_non_existent_fails() {
    let env = Env::default();
    let contract_id = env.register(AutoShareContract, ());
    let client = AutoShareContractClient::new(&env, &contract_id);

    let id = BytesN::from_array(&env, &[9u8; 32]);
    client.get(&id);
}