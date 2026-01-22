#![no_std]
use soroban_sdk::{contract, contractimpl, Env, BytesN, String, Address};

// 1. Declare the foundational modules (Requirement: Modular Structure)
pub mod base {
    pub mod types;
    pub mod events;
    pub mod errors;
}

pub mod interfaces {
    pub mod autoshare;
}

// 2. Declare the main logic file where the functions are implemented
mod autoshare_logic;

#[contract]
pub struct AutoShareContract;

#[contractimpl]
impl AutoShareContract {
    /// Creates a new AutoShare plan.
    /// Requirement: create_autoshare should store data and emit an event.
    pub fn create(env: Env, id: BytesN<32>, name: String, creator: Address) {
        autoshare_logic::create_autoshare(env, id, name, creator).unwrap();
    }

    /// Retrieves an existing AutoShare plan.
    /// Requirement: get_autoshare should return the plan details.
    pub fn get(env: Env, id: BytesN<32>) -> base::types::AutoShareDetails {
        autoshare_logic::get_autoshare(env, id).unwrap()
    }
}

// 3. Link the tests (Requirement: Unit Tests)
#[cfg(test)]
#[path = "tests/autoshare_test.rs"]
mod autoshare_test; // Links the internal tests/autoshare_test.rs inside src