#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct TokenTransferContract;

#[contractimpl]
impl TokenTransferContract {
    pub fn transfer(_env: Env, _from: Address, _to: Address, _amount: i128) {
        // dummy implementation
    }
}

#[cfg(test)]
mod test;
