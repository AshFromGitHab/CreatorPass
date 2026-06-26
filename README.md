# CreatorPass

**Time-locked USDC subscriptions for Filipino content creators — fans pay once, creators claim at month-end with full on-chain transparency.**

---

## Problem

A Filipino Twitch streamer with 8,000 followers gets paid via PayPal subscriptions from fans in Singapore and the US. PayPal charges 4.4% + $0.30 per transaction, withholds payouts for 21 days for risk review, and the streamer can't receive at all if PayPal decides to limit their account — which happens to Philippine users disproportionately.

## Solution

Fans subscribe by locking USDC into **GigLink**, a Soroban smart contract. After 30 days, the creator calls `claim` and receives the USDC directly to their Stellar wallet. Zero middleman. Sub-cent fees. The fan can cancel within the first day if the creator goes inactive, protecting against abandoned channels.

## Stellar Features Used

- **USDC transfers** — subscription payments locked in contract escrow
- **Soroban smart contracts** — time-lock logic, cancel/refund, access control
- **Trustlines** — creator and fan both hold USDC trustlines

## Target Users

- Filipino content creators (YouTube, Twitch, TikTok) earning $100–$2,000/month
- Their fans in Singapore, Hong Kong, Australia, and the Philippines
- Indie artists, podcast hosts, educators selling monthly access

## Vision and Purpose

GigLink is Patreon built on Stellar — but without Patreon's 8% cut, 30-day payout delays, and geographic discrimination. Every creator with a Stellar wallet can accept global fan subscriptions. The on-chain model is composable: add subscription tiers, NFT access passes, or DeFi yield on locked funds.

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
  --wasm target/wasm32-unknown-unknown/release/giglink.wasm \
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

## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CBKWA565RWKBI55KVLECNZVDQ4J4YGOAO523EXZAEQBENYURPDNHR7S4` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CBKWA565RWKBI55KVLECNZVDQ4J4YGOAO523EXZAEQBENYURPDNHR7S4) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/8619c90818130f5cf2b4d173c89bdf666618f250647e324c22545e322467b112) |
| Deployed | 2026-06-26 06:53:50 UTC |
| Wallet | freighter (`GCEW…IHMP`) |
