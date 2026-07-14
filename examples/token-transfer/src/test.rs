#![cfg(test)]

use super::*;
use soroban_meter::MeterExt;
use soroban_sdk::{Address, Env};

#[test]
fn test_transfer_resource_usage() {
    let env = Env::default();
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    // Reset budget before the call we want to measure
    env.budget().reset_default();

    // Invoke the contract function
    let client = TokenTransferContractClient::new(&env, &env.register(TokenTransferContract, ()));
    client.transfer(&from, &to, &1000);

    // Print the full resource breakdown
    env.meter_report("transfer");
}
