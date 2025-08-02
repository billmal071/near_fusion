#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext, Balance};
    use escrow::{Immutables, Timelocks};

    const NEAR: Balance = 1_000_000_000_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id)
            .attached_deposit(5 * NEAR) // For contract deployment
            .build()
    }

    fn create_test_immutables() -> Immutables {
        Immutables {
            order_hash: [1; 32],
            hashlock: [2; 32],
            maker: accounts(1),
            taker: accounts(2),
            token: accounts(3),
            amount: U128(1000),
            safety_deposit: U128(100),
            timelocks: Timelocks {
                src_withdrawal: 3600,
                src_public_withdrawal: 7200,
                src_cancellation: 10800,
                src_public_cancellation: 14400,
                dst_withdrawal: 1800,
                dst_public_withdrawal: 3600,
                dst_cancellation: 7200,
                deployed_at: 0,
            },
        }
    }

    #[test]
    fn test_factory_initialization() {
        let context = get_context(accounts(0));
        testing_env!(context);
        
        let src_code = vec![1, 2, 3]; // Mock WASM code
        let dst_code = vec![4, 5, 6]; // Mock WASM code
        
        let factory = EscrowFactory::new(src_code.clone(), dst_code.clone());
        
        assert_eq!(factory.escrow_src_code, src_code);
        assert_eq!(factory.escrow_dst_code, dst_code);
    }

    #[test]
    fn test_create_src_escrow() {
        let mut context = get_context(accounts(0));
        testing_env!(context);
        
        let src_code = vec![1, 2, 3];
        let dst_code = vec![4, 5, 6];
        
        let mut factory = EscrowFactory::new(src_code, dst_code);
        let immutables = create_test_immutables();
        
        // Calculate expected address
        let salt = immutables.hash();
        let expected_address = factory._get_escrow_account_id(&salt, "src");
        
        // Create escrow
        let escrow_address = factory.create_src_escrow(immutables.clone());
        
        // Verify deployment recorded
        assert_eq!(factory.get_deployment(salt), Some(expected_address.clone()));
        assert_eq!(escrow_address, expected_address);
    }

    #[test]
    fn test_create_dst_escrow() {
        let mut context = get_context(accounts(2)); // Taker account
        testing_env!(context);
        
        let src_code = vec![1, 2, 3];
        let dst_code = vec![4, 5, 6];
        
        let mut factory = EscrowFactory::new(src_code, dst_code);
        let immutables = create_test_immutables();
        
        // Calculate expected address
        let salt = immutables.hash();
        let expected_address = factory._get_escrow_account_id(&salt, "dst");
        
        // Create escrow with valid cancellation timestamp
        let src_cancellation_timestamp = 20000; // After dst_cancellation (7200)
        let escrow_address = factory.create_dst_escrow(immutables.clone(), src_cancellation_timestamp);
        
        // Verify deployment recorded
        assert_eq!(factory.get_deployment(salt), Some(expected_address.clone()));
        assert_eq!(escrow_address, expected_address);
    }

    #[test]
    #[should_panic(expected = "Only taker can create dst escrow")]
    fn test_create_dst_escrow_wrong_caller() {
        let mut context = get_context(accounts(1)); // Maker account (wrong)
        testing_env!(context);
        
        let src_code = vec![1, 2, 3];
        let dst_code = vec![4, 5, 6];
        
        let mut factory = EscrowFactory::new(src_code, dst_code);
        let immutables = create_test_immutables();
        
        factory.create_dst_escrow(immutables, 20000);
    }

    #[test]
    #[should_panic(expected = "Invalid creation time")]
    fn test_create_dst_escrow_invalid_time() {
        let mut context = get_context(accounts(2)); // Taker account
        testing_env!(context);
        
        let src_code = vec![1, 2, 3];
        let dst_code = vec![4, 5, 6];
        
        let mut factory = EscrowFactory::new(src_code, dst_code);
        let immutables = create_test_immutables();
        
        // src_cancellation_timestamp is before dst_cancellation
        let src_cancellation_timestamp = 5000; // Before dst_cancellation (7200)
        factory.create_dst_escrow(immutables, src_cancellation_timestamp);
    }

    #[test]
    #[should_panic(expected = "Escrow already deployed")]
    fn test_duplicate_deployment() {
        let mut context = get_context(accounts(0));
        testing_env!(context);
        
        let src_code = vec![1, 2, 3];
        let dst_code = vec![4, 5, 6];
        
        let mut factory = EscrowFactory::new(src_code, dst_code);
        let immutables = create_test_immutables();
        
        // First deployment should succeed
        factory.create_src_escrow(immutables.clone());
        
        // Second deployment with same immutables should fail
        factory.create_src_escrow(immutables);
    }

    #[test]
    fn test_deterministic_addresses() {
        let context = get_context(accounts(0));
        testing_env!(context);
        
        let src_code = vec![1, 2, 3];
        let dst_code = vec![4, 5, 6];
        
        let factory = EscrowFactory::new(src_code, dst_code);
        let immutables = create_test_immutables();
        
        let salt = immutables.hash();
        
        // Get addresses
        let src_address = factory._get_escrow_account_id(&salt, "src");
        let dst_address = factory._get_escrow_account_id(&salt, "dst");
        
        // Verify they're different
        assert_ne!(src_address, dst_address);
        
        // Verify format
        assert!(src_address.to_string().starts_with("src-"));
        assert!(dst_address.to_string().starts_with("dst-"));
        assert!(src_address.to_string().contains(&env::current_account_id().to_string()));
    }
}