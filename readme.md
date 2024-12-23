Staking Smart Contract (StakeX)
This is a decentralized staking contract for the MultiversX blockchain. It allows users to stake EGLD tokens, earn rewards based on the Annual Percentage Yield (APY), claim rewards, and withdraw staked tokens in a decentralized and trustless manner.

Table of Contents
Project Overview
Features
Installation
Contract Deployment
Usage
Testnet Deployment
Contributing
License
Project Overview
This project consists of a Staking Smart Contract deployed on the MultiversX Testnet. It supports the following features:

Stake EGLD tokens
Unstake partially or fully
Claim rewards based on the staked amount and staking duration
Calculate rewards using a predefined APY
Display real-time balances and staking details
Features
The Staking Smart Contract has the following key functions:

stake: Allows users to deposit EGLD tokens for staking.
unstake: Lets users withdraw staked tokens, either partially or fully.
claimRewards: Users can claim rewards without unstaking.
calculateRewards: Calculates rewards based on the staking duration and the current APY.
getStakedAddresses: Retrieves a list of addresses that have staked tokens.
getStakingPosition: Returns the staking details (e.g., staked amount and last action block) for a specific user.
getApy: Provides the current APY set in the contract.
User Interface (UI)
Connect Wallet: Users can connect a MultiversX-compatible wallet (e.g., Maiar Wallet).
Staking Dashboard: Display current staking details, including staked amount and accumulated rewards.
Stake Tokens: Users can deposit EGLD tokens to stake.
Unstake Tokens: Allows users to withdraw EGLD tokens.
Claim Rewards: Lets users claim rewards earned from staking.
