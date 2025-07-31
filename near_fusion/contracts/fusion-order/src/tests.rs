#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: AccountId, block_timestamp: u64) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id)
            .block_timestamp(block_timestamp)
            .build()
    }

    fn create_test_order() -> FusionOrder {
        FusionOrder {
            order_hash: [1; 32],
            maker: accounts(1),
            resolver: Some(accounts(3)),
            maker_asset: "token1.near".parse().unwrap(),
            taker_asset: "token2.near".parse().unwrap(),
            making_amount: U128(1000),
            taking_amount: U128(2000),
            maker_traits: 0,
            salt: [2; 32],
        }
    }

    fn create_test_auction() -> AuctionDetails {
        AuctionDetails {
            start_time: 1000,
            duration: 3600,
            initial_rate_bump: 100, // 1%
            points: vec![
                AuctionPoint { delay: 900, coefficient: 750000 }, // 75% after 15 min
                AuctionPoint { delay: 1800, coefficient: 500000 }, // 50% after 30 min
            ],
        }
    }

    #[test]
    fn test_create_order() {
        let context = get_context(accounts(1), 0);
        testing_env!(context);
        
        let mut contract = FusionOrderContract::new(accounts(5), 30); // 0.3% fee
        let order = create_test_order();
        
        let order_hash = contract.create_order(order.clone(), None);
        assert_eq!(order_hash, order.hash());
        
        let stored_order = contract.get_order(order_hash).unwrap();
        assert_eq!(stored_order.order.maker, accounts(1));
        assert_eq!(stored_order.filled_making_amount, 0);
        assert!(!stored_order.is_cancelled);
    }

    #[test]
    fn test_partial_fill_validation() {
        let order = create_test_order();
        
        // Valid partial fill (50%)
        assert!(order.is_valid_partial_fill(500, 1000));
        
        // Invalid partial fill (wrong ratio)
        assert!(!order.is_valid_partial_fill(500, 900));
        
        // Invalid partial fill (zero amounts)
        assert!(!order.is_valid_partial_fill(0, 0));
        assert!(!order.is_valid_partial_fill(500, 0));
        assert!(!order.is_valid_partial_fill(0, 1000));
    }

    #[test]
    fn test_dutch_auction_rate_bump() {
        let auction = create_test_auction();
        
        // Before auction starts
        assert_eq!(auction.get_rate_bump(500), 100);
        
        // At auction start
        assert_eq!(auction.get_rate_bump(1000), 100);
        
        // After 15 minutes (900 seconds)
        assert_eq!(auction.get_rate_bump(1900), 75);
        
        // After 30 minutes (1800 seconds)
        assert_eq!(auction.get_rate_bump(2800), 37);
        
        // After auction ends
        assert_eq!(auction.get_rate_bump(5000), 0);
    }

    #[test]
    fn test_cancel_order() {
        let context = get_context(accounts(1), 0);
        testing_env!(context);
        
        let mut contract = FusionOrderContract::new(accounts(5), 30);
        let order = create_test_order();
        
        let order_hash = contract.create_order(order, None);
        contract.cancel_order(order_hash);
        
        let stored_order = contract.get_order(order_hash).unwrap();
        assert!(stored_order.is_cancelled);
    }

    #[test]
    #[should_panic(expected = "Only maker can cancel")]
    fn test_cancel_order_unauthorized() {
        let mut context = get_context(accounts(1), 0);
        testing_env!(context);
        
        let mut contract = FusionOrderContract::new(accounts(5), 30);
        let order = create_test_order();
        
        let order_hash = contract.create_order(order, None);
        
        // Try to cancel as different account
        context = get_context(accounts(2), 0);
        testing_env!(context);
        
        contract.cancel_order(order_hash);
    }

    #[test]
    fn test_resolver_management() {
        let context = get_context("contract.near".parse().unwrap(), 0);
        testing_env!(context);
        
        let mut contract = FusionOrderContract::new(accounts(5), 30);
        
        contract.add_resolver(accounts(3));
        assert!(contract.resolver_whitelist.get(&accounts(3)).unwrap_or(false));
        
        contract.remove_resolver(accounts(3));
        assert!(!contract.resolver_whitelist.get(&accounts(3)).unwrap_or(false));
    }

    #[test]
    fn test_get_remaining_amounts() {
        let context = get_context(accounts(1), 0);
        testing_env!(context);
        
        let mut contract = FusionOrderContract::new(accounts(5), 30);
        let order = create_test_order();
        
        let order_hash = contract.create_order(order, None);
        
        let (remaining_making, remaining_taking) = contract.get_remaining_amounts(order_hash);
        assert_eq!(remaining_making.0, 1000);
        assert_eq!(remaining_taking.0, 2000);
    }
}