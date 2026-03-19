#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Vec, Symbol
};

#[contracttype]
#[derive(Clone)]
pub struct AirdropRecord {
    pub recipient: Address,
    pub amount: i128,
}

#[contract]
pub struct AirdropContract;

#[contractimpl]
impl AirdropContract {

    // Airdrop tokens to multiple recipients
    pub fn airdrop(
        env: Env,
        token: Address,
        from: Address,
        recipients: Vec<AirdropRecord>,
    ) {
        // Require auth from sender
        from.require_auth();

        let token_client = soroban_sdk::token::Client::new(&env, &token);

        for record in recipients.iter() {
            token_client.transfer(
                &from,
                &record.recipient,
                &record.amount,
            );
        }
    }

    // Simple helper: single transfer
    pub fn single_transfer(
        env: Env,
        token: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        from.require_auth();

        let token_client = soroban_sdk::token::Client::new(&env, &token);

        token_client.transfer(&from, &to, &amount);
    }
}
