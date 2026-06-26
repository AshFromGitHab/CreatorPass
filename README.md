# CreatorPass

**Time-locked USDC subscriptions for Filipino content creators — fans pay once, creators claim at month-end with full on-chain transparency.**

---

## Problem

A Filipino Twitch streamer with 8,000 followers gets paid via PayPal subscriptions from fans in Singapore and the US. PayPal charges 4.4% + $0.30 per transaction, withholds payouts for 21 days for risk review, and the streamer can't receive at all if PayPal decides to limit their account — which happens to Philippine users disproportionately.

## Solution

Fans subscribe by locking USDC into CreatorPass, a Soroban contract. After 30 days, the creator calls `claim` and receives the USDC directly to their Stellar wallet. Zero middleman. Sub-cent fees. The fan can cancel within the first day if the creator goes inactive, protecting against abandoned channels.

## Stellar Features Used

- **USDC transfers** — subscription payments locked in contract escrow
- **Soroban smart contracts** — time-lock logic, cancel/refund, access control
- **Trustlines** — creator and fan both hold USDC trustline

## Target Users

- Filipino content creators (YouTube, Twitch, TikTok) earning $100–$2,000/month
- Their fans in Singapore, Hong Kong, Australia, and the Philippines
- Indie artists, podcast hosts, educators selling monthly access

## Vision and Purpose

CreatorPass is Patreon built on Stellar — but without Patreon's 8% cut, 30-day payout delays, and geographic discrimination. Every creator with a Stellar wallet can accept global fan subscriptions. The on-chain model is composable: add tiers, NFT access passes, or DeFi yield on locked funds.

---

## Prerequisites

- Rust (latest stable)
- Soroban CLI v21+

## Build

```bash
soroban contract build
```

## Test

```bash
cargo test
```

## Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/creatorpass.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet
```

## Initialize

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  --network testnet \
  -- initialize \
  --usdc_token USDC_CONTRACT_ADDRESS
```

## Fan Subscribes

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source FAN_SECRET_KEY \
  --network testnet \
  -- subscribe \
  --fan GB_FAN_ADDRESS \
  --creator GB_CREATOR_ADDRESS \
  --monthly_amount 50000000
```

## Creator Claims After 30 Days

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source CREATOR_SECRET_KEY \
  --network testnet \
  -- claim \
  --fan GB_FAN_ADDRESS \
  --creator GB_CREATOR_ADDRESS
```

---

## License

MIT
