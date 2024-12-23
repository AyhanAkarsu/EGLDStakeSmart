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
    staked_addresses: Mapping<u64, Address>,    // Stake yapan adreslerin listesi
    staking_positions: Mapping<Address, StakingPosition>, // Kullanıcıların stake pozisyonları
}

#[derive(Clone)]
pub struct StakingPosition {
    pub amount_staked: BigUint, // Kullanıcının stake ettiği miktar
    pub last_staked_block: u64, // Son stake edilen blok numarası
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

        // Kullanıcı stake pozisyonunu güncelle
        let current_position = StakingPosition {
            amount_staked: current_staked.clone(),
            last_staked_block: ctx.block_number(),
        };
        self.staking_positions.insert(sender.clone(), current_position);

        // Stake yapılan adresi kaydet
        let stake_count = self.staked_addresses.len();
        self.staked_addresses.insert(stake_count as u64, sender.clone());

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

    // Stake yapan adreslerin listesini döndürür
    pub fn get_staked_addresses(&self) -> Vec<Address> {
        let mut addresses = Vec::new();
        for i in 0..self.staked_addresses.len() {
            let address = self.staked_addresses.get(i as u64);
            addresses.push(address);
        }
        addresses
    }

    // Kullanıcıların stake pozisyonlarını döndürür (stake edilen miktar ve son işlem bloğu)
    pub fn get_staking_position(&self, user: Address) -> Option<StakingPosition> {
        self.staking_positions.get(user)
    }

    // APY'yi almak için fonksiyon
    pub fn get_apy(&self) -> u64 {
        self.apy
    }
}
