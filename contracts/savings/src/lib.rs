#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct SavingsContract;

#[contractimpl]
impl SavingsContract {
    pub fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth();

        assert!(amount > 0, "amount must be positive");

        let _ = env;
    }

    pub fn withdraw(env: Env, user: Address, amount: i128) {
        user.require_auth();

        assert!(amount > 0, "amount must be positive");

        let _ = env;
    }

    pub fn balance(env: Env, user: Address) -> i128 {
        let _ = env;
        let _ = user;
        0
    }
}
