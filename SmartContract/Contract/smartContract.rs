#![no_std]

use multiversx_sdk::contract::{
    contract,
    execution_context::ExecutionContext,
    storage::{Mapping},
    types::{BigUint, Address},
};

#[contract]
pub struct StakingContract {
    staked_tokens: Mapping<Address, BigUint>,  // Kullanıcıların stake ettiği tokenlar
    staking_duration: Mapping<Address, u64>,    // Kullanıcıların stake süresi (blok sayısı)
    apy: u64,                                    // Yıllık getiri oranı (APY)
}

impl StakingContract {
    // Stake fonksiyonu: Kullanıcılar EGLD tokenlerini stake eder
    #[payable]
    pub fn stake(ctx: &mut ExecutionContext, amount: BigUint) -> Result<(), String> {
        let sender = ctx.sender().to_address();
        let current_balance = ctx.balance_of(sender.clone());

        // Kullanıcı yeterli bakiyeye sahip mi?
        if amount > current_balance {
            return Err("Not enough balance.".to_string());
        }

        // Kullanıcı stake miktarını kaydediyor
        let mut current_staked = self.staked_tokens.get(sender.clone());
        current_staked += amount.clone();
        self.staked_tokens.insert(sender.clone(), current_staked);

        // Stake süresi kaydedilir (blok numarası üzerinden)
        self.staking_duration.insert(sender.clone(), ctx.block_number());

        Ok(())
    }

    // Unstake fonksiyonu: Kullanıcılar stake ettikleri tokenları geri çekebilir
    pub fn unstake(ctx: &mut ExecutionContext, amount: BigUint) -> Result<(), String> {
        let sender = ctx.sender().to_address();
        let staked_balance = self.staked_tokens.get(sender.clone());

        // Kullanıcı yeterli stake miktarına sahip mi?
        if amount > staked_balance {
            return Err("Not enough staked tokens.".to_string());
        }

        // Kullanıcıya tokenleri iade et
        ctx.transfer_from_contract(sender.clone(), amount.clone())?;

        // Stake miktarını güncelle
        self.staked_tokens.insert(sender.clone(), staked_balance - amount.clone());

        Ok(())
    }

    // Ödül hesaplama fonksiyonu: Stake edilen miktar ve süreye göre ödülleri hesaplar
    pub fn calculate_rewards(&self, staked_amount: BigUint, staking_duration: u64) -> BigUint {
        // Basit APY ödül hesaplaması
        let yearly_rewards = staked_amount * self.apy;
        let rewards = yearly_rewards * staking_duration / 365;  // Yıllık oran üzerinden hesaplama
        rewards
    }

    // Claim Rewards: Kullanıcıların ödüllerini talep etmelerini sağlar
    pub fn claim_rewards(&self, ctx: &mut ExecutionContext) -> Result<BigUint, String> {
        let sender = ctx.sender().to_address();
        let staked_amount = self.staked_tokens.get(sender.clone());
        let staking_duration = self.staking_duration.get(sender.clone());

        let rewards = self.calculate_rewards(staked_amount, staking_duration);

        // Ödülleri kullanıcıya gönder
        ctx.transfer_from_contract(sender.clone(), rewards.clone())?;
        Ok(rewards)
    }

    // APY'yi almak için fonksiyon
    pub fn get_apy(&self) -> u64 {
        self.apy
    }
}
