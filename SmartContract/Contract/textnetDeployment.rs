#![no_std]

use multiversx_sc::api::{self, contract_api};
use multiversx_sc::types::{BigUint, Address};
use multiversx_sc::storage::Storage;

// Define storage variables
#[multiversx_sc::contract]
pub trait StakingContract {
    #[storage_mapper("staked_balance")]
    fn staked_balance(&self, address: &Address) -> BigUint;

    #[storage_mapper("reward_balance")]
    fn reward_balance(&self, address: &Address) -> BigUint;

    #[init]
    fn init(&self) {
        // Initialization code (if necessary)
    }

    #[endpoint]
    fn stake(&self, amount: BigUint) {
        let caller = contract_api::caller();
        let current_balance = self.staked_balance(&caller);
        self.staked_balance(&caller).set(&current_balance + amount);
    }

    #[endpoint]
    fn unstake(&self, amount: BigUint) {
        let caller = contract_api::caller();
        let current_balance = self.staked_balance(&caller);
        self.staked_balance(&caller).set(&(current_balance - amount));
    }

    #[endpoint]
    fn claim_rewards(&self) {
        let caller = contract_api::caller();
        let rewards = self.reward_balance(&caller);
        self.reward_balance(&caller).set(&BigUint::from(0u32));
        // Transfer the rewards to the user
        contract_api::transfer_to_address(&caller, rewards);
    }

    #[endpoint]
    fn calculate_rewards(&self) -> BigUint {
        // A placeholder function to calculate rewards
        let caller = contract_api::caller();
        let staked_amount = self.staked_balance(&caller);
        BigUint::from(staked_amount * 10u32) // Example APY formula
    }

    #[view]
    fn get_staked_balance(&self) -> BigUint {
        let caller = contract_api::caller();
        self.staked_balance(&caller)
    }

    #[view]
    fn get_reward_balance(&self) -> BigUint {
        let caller = contract_api::caller();
        self.reward_balance(&caller)
    }
}
