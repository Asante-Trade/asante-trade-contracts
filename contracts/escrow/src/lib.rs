#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn create(env: Env, buyer: Address, seller: Address, amount: i128) {
        buyer.require_auth();

        assert!(amount > 0, "amount must be positive");

        let _ = env;
        let _ = seller;
    }

    pub fn release(env: Env, buyer: Address, seller: Address) {
        buyer.require_auth();

        let _ = env;
        let _ = seller;
    }

    pub fn refund(env: Env, buyer: Address) {
        buyer.require_auth();

        let _ = env;
    }
}
