#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct LoansContract;

#[contractimpl]
impl LoansContract {
    pub fn create_loan(env: Env, borrower: Address, amount: i128) {
        borrower.require_auth();

        assert!(amount > 0, "amount must be positive");

        let _ = env;
    }

    pub fn repay(env: Env, borrower: Address, amount: i128) {
        borrower.require_auth();

        assert!(amount > 0, "amount must be positive");

        let _ = env;
    }

    pub fn outstanding(env: Env, borrower: Address) -> i128 {
        let _ = env;
        let _ = borrower;
        0
    }
}
