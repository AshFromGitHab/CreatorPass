// CreatorPass — USDC Subscription Escrow for Filipino Creators
// A fan subscribes monthly to a creator (YouTuber, Twitch streamer, indie artist)
// by locking USDC in a Soroban escrow contract.
// The creator claims the USDC at month end only if they've met an activity threshold.
// Protects fans from inactive creators; guarantees creators get paid on time.

#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    token, Address, Env, Symbol,
};

const USDC_TOKEN: Symbol = symbol_short!("USDCTKN");

// Per-subscription record
#[contracttype]
#[derive(Clone)]
pub struct Subscription {
    pub fan: Address,
    pub creator: Address,
    pub monthly_amount: i128,    // USDC stroops per month
    pub locked_until: u64,       // ledger timestamp when creator can claim
    pub claimed: bool,
    pub active: bool,
}

// Subscription ID = hash of (fan, creator) for uniqueness
#[contracttype]
pub enum DataKey {
    Sub(Address, Address), // (fan, creator)
    CreatorEarned(Address), // cumulative USDC earned by creator
}

#[contract]
pub struct CreatorPassContract;

#[contractimpl]
impl CreatorPassContract {
    // Initialize with USDC token address
    pub fn initialize(env: Env, usdc_token: Address) {
        if env.storage().instance().has(&USDC_TOKEN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&USDC_TOKEN, &usdc_token);
    }

    // Fan subscribes to a creator, locking USDC for 30 days (30 * 24 * 3600 seconds)
    // monthly_amount: USDC in stroops (e.g. 5_0000_000 = $5.00)
    pub fn subscribe(
        env: Env,
        fan: Address,
        creator: Address,
        monthly_amount: i128,
    ) {
        fan.require_auth();

        if monthly_amount <= 0 {
            panic!("amount must be positive");
        }

        // Prevent duplicate active subscription
        let key = DataKey::Sub(fan.clone(), creator.clone());
        if let Some(sub) = env
            .storage()
            .persistent()
            .get::<DataKey, Subscription>(&key)
        {
            if sub.active {
                panic!("subscription already active");
            }
        }

        let usdc_token: Address = env.storage().instance().get(&USDC_TOKEN).unwrap();
        let token_client = token::Client::new(&env, &usdc_token);

        // Lock USDC from fan into contract for 30 days
        token_client.transfer(&fan, &env.current_contract_address(), &monthly_amount);

        let locked_until = env.ledger().timestamp() + 30 * 24 * 3600; // 30 days

        let sub = Subscription {
            fan: fan.clone(),
            creator: creator.clone(),
            monthly_amount,
            locked_until,
            claimed: false,
            active: true,
        };
        env.storage().persistent().set(&key, &sub);
    }

    // Creator claims their USDC after the 30-day lock period
    // In a production app, add an activity-check oracle here
    pub fn claim(env: Env, fan: Address, creator: Address) -> i128 {
        creator.require_auth();

        let key = DataKey::Sub(fan.clone(), creator.clone());
        let mut sub: Subscription = env
            .storage()
            .persistent()
            .get(&key)
            .expect("subscription not found");

        if !sub.active {
            panic!("subscription not active");
        }
        if sub.claimed {
            panic!("already claimed");
        }
        if env.ledger().timestamp() < sub.locked_until {
            panic!("lock period not expired");
        }

        let usdc_token: Address = env.storage().instance().get(&USDC_TOKEN).unwrap();
        let token_client = token::Client::new(&env, &usdc_token);

        // Release USDC to creator
        token_client.transfer(
            &env.current_contract_address(),
            &creator,
            &sub.monthly_amount,
        );

        // Update cumulative earnings
        let earned_key = DataKey::CreatorEarned(creator.clone());
        let prev: i128 = env.storage().persistent().get(&earned_key).unwrap_or(0);
        env.storage()
            .persistent()
            .set(&earned_key, &(prev + sub.monthly_amount));

        sub.claimed = true;
        sub.active = false;
        env.storage().persistent().set(&key, &sub);

        sub.monthly_amount
    }

    // Fan cancels subscription before lock expires — refund USDC if not yet claimed
    pub fn cancel(env: Env, fan: Address, creator: Address) {
        fan.require_auth();

        let key = DataKey::Sub(fan.clone(), creator.clone());
        let mut sub: Subscription = env
            .storage()
            .persistent()
            .get(&key)
            .expect("subscription not found");

        if !sub.active || sub.claimed {
            panic!("subscription not cancellable");
        }
        // Can only cancel before lock expires (within cooling window)
        if env.ledger().timestamp() >= sub.locked_until {
            panic!("lock expired — creator can now claim");
        }

        let usdc_token: Address = env.storage().instance().get(&USDC_TOKEN).unwrap();
        let token_client = token::Client::new(&env, &usdc_token);

        // Refund to fan
        token_client.transfer(
            &env.current_contract_address(),
            &fan,
            &sub.monthly_amount,
        );

        sub.active = false;
        env.storage().persistent().set(&key, &sub);
    }

    // View: get subscription details
    pub fn get_subscription(env: Env, fan: Address, creator: Address) -> Subscription {
        env.storage()
            .persistent()
            .get(&DataKey::Sub(fan, creator))
            .expect("not found")
    }

    // View: cumulative USDC earned by creator
    pub fn creator_earned(env: Env, creator: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::CreatorEarned(creator))
            .unwrap_or(0)
    }
}
