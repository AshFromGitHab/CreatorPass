#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        token::{Client as TokenClient, StellarAssetClient},
        Address, Env,
    };

    fn setup() -> (Env, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let fan = Address::generate(&env);
        let creator = Address::generate(&env);

        let token_admin = Address::generate(&env);
        let usdc_id = env.register_stellar_asset_contract_v2(token_admin.clone());
        let usdc_address = usdc_id.address();

        // Mint 1,000 USDC to fan
        let stellar_client = StellarAssetClient::new(&env, &usdc_address);
        stellar_client.mint(&fan, &10_000_000_000i128);

        (env, fan, creator, usdc_address)
    }

    // Test 1 — Happy path: fan subscribes, 30 days pass, creator claims
    #[test]
    fn test_subscribe_and_claim() {
        let (env, fan, creator, usdc_address) = setup();

        let contract_id = env.register_contract(None, CreatorPassContract);
        let client = CreatorPassContractClient::new(&env, &contract_id);

        client.initialize(&usdc_address);

        let amount = 50_000_000i128; // $5.00

        env.ledger().with_mut(|l| l.timestamp = 1_000_000);
        client.subscribe(&fan, &creator, &amount);

        // Advance 31 days
        env.ledger().with_mut(|l| l.timestamp = 1_000_000 + 31 * 24 * 3600);

        let claimed = client.claim(&fan, &creator);
        assert_eq!(claimed, amount);

        let token_client = TokenClient::new(&env, &usdc_address);
        assert_eq!(token_client.balance(&creator), amount);
    }

    // Test 2 — Edge case: creator cannot claim before 30 days
    #[test]
    #[should_panic(expected = "lock period not expired")]
    fn test_claim_too_early_fails() {
        let (env, fan, creator, usdc_address) = setup();

        let contract_id = env.register_contract(None, CreatorPassContract);
        let client = CreatorPassContractClient::new(&env, &contract_id);

        client.initialize(&usdc_address);

        env.ledger().with_mut(|l| l.timestamp = 1_000_000);
        client.subscribe(&fan, &creator, &50_000_000i128);

        // Only 5 days have passed — must panic
        env.ledger().with_mut(|l| l.timestamp = 1_000_000 + 5 * 24 * 3600);
        client.claim(&fan, &creator);
    }

    // Test 3 — State verification: subscription marked claimed after successful claim
    #[test]
    fn test_subscription_state_after_claim() {
        let (env, fan, creator, usdc_address) = setup();

        let contract_id = env.register_contract(None, CreatorPassContract);
        let client = CreatorPassContractClient::new(&env, &contract_id);

        client.initialize(&usdc_address);

        env.ledger().with_mut(|l| l.timestamp = 500_000);
        client.subscribe(&fan, &creator, &50_000_000i128);
        env.ledger().with_mut(|l| l.timestamp = 500_000 + 31 * 24 * 3600);
        client.claim(&fan, &creator);

        let sub = client.get_subscription(&fan, &creator);
        assert_eq!(sub.claimed, true);
        assert_eq!(sub.active, false);
    }

    // Test 4 — Cumulative earnings tracked correctly across multiple subscriptions
    #[test]
    fn test_creator_earned_accumulates() {
        let (env, fan, creator, usdc_address) = setup();
        let fan2 = Address::generate(&env);

        let stellar_client = StellarAssetClient::new(&env, &usdc_address);
        stellar_client.mint(&fan2, &10_000_000_000i128);

        let contract_id = env.register_contract(None, CreatorPassContract);
        let client = CreatorPassContractClient::new(&env, &contract_id);

        client.initialize(&usdc_address);

        env.ledger().with_mut(|l| l.timestamp = 100_000);
        client.subscribe(&fan, &creator, &50_000_000i128);
        client.subscribe(&fan2, &creator, &100_000_000i128);

        env.ledger().with_mut(|l| l.timestamp = 100_000 + 31 * 24 * 3600);
        client.claim(&fan, &creator);
        client.claim(&fan2, &creator);

        let earned = client.creator_earned(&creator);
        assert_eq!(earned, 150_000_000i128);
    }

    // Test 5 — Fan can cancel before lock expires and get refund
    #[test]
    fn test_fan_cancel_refunds() {
        let (env, fan, creator, usdc_address) = setup();

        let contract_id = env.register_contract(None, CreatorPassContract);
        let client = CreatorPassContractClient::new(&env, &contract_id);

        client.initialize(&usdc_address);

        let token_client = TokenClient::new(&env, &usdc_address);
        let balance_before = token_client.balance(&fan);

        env.ledger().with_mut(|l| l.timestamp = 1_000_000);
        client.subscribe(&fan, &creator, &50_000_000i128);

        // Cancel after 1 day (within lock window)
        env.ledger().with_mut(|l| l.timestamp = 1_000_000 + 86_400);
        client.cancel(&fan, &creator);

        // Fan should be refunded fully
        assert_eq!(token_client.balance(&fan), balance_before);
    }
}
