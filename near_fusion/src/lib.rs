use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, PanicOnDefault, Promise,
};
use sha3::{Digest, Keccak256};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Timelocks {
    pub src_withdrawal: u64,
    pub src_public_withdrawal: u64,
    pub src_cancellation: u64,
    pub src_public_cancellation: u64,
    pub dst_withdrawal: u64,
    pub dst_public_withdrawal: u64,
    pub dst_cancellation: u64,
    pub deployed_at: u64,
}

impl Timelocks {
    pub fn is_stage_time(&self, stage: Stage, time_bound_kind: TimeBoundKind) -> bool {
        let time_bound_timestamp = match stage {
            Stage::SrcWithdrawal => self.deployed_at + self.src_withdrawal,
            Stage::SrcPublicWithdrawal => self.deployed_at + self.src_public_withdrawal,
            Stage::SrcCancellation => self.deployed_at + self.src_cancellation,
            Stage::SrcPublicCancellation => self.deployed_at + self.src_public_cancellation,
            Stage::DstWithdrawal => self.deployed_at + self.dst_withdrawal,
            Stage::DstPublicWithdrawal => self.deployed_at + self.dst_public_withdrawal,
            Stage::DstCancellation => self.deployed_at + self.dst_cancellation,
        };
        let current_timestamp = env::block_timestamp();
        match time_bound_kind {
            TimeBoundKind::Before => current_timestamp < time_bound_timestamp,
            TimeBoundKind::After => current_timestamp >= time_bound_timestamp,
        }
    }
}

#[derive(Clone)]
pub enum Stage {
    SrcWithdrawal,
    SrcPublicWithdrawal,
    SrcCancellation,
    SrcPublicCancellation,
    DstWithdrawal,
    DstPublicWithdrawal,
    DstCancellation,
}

#[derive(Clone)]
pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EscrowOrder {
    pub order_hash: [u8; 32],
    pub secret_hash: [u8; 32],
    pub maker: AccountId,
    pub taker: AccountId,
    pub token_contract: AccountId,
    pub amount: U128,
    pub safety_deposit: U128,
    pub timelocks: Timelocks,
    pub is_active: bool,
}

#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_transfer_from(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    );
}

const GAS_FOR_FT_TRANSFER: u64 = 5_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FusionEscrow {
    escrows: UnorderedMap<[u8; 32], EscrowOrder>,
    escrow_count: u64,
}

#[near_bindgen]
impl FusionEscrow {
    #[init]
    pub fn new() -> Self {
        Self {
            escrows: UnorderedMap::new(b"e"),
            escrow_count: 0,
        }
    }

    #[payable]
    pub fn deposit(&mut self, escrow_order: EscrowOrder) -> [u8; 32] {
        let maker = env::predecessor_account_id();
        assert_eq!(escrow_order.maker, maker, "Only maker can deposit");
        assert!(!escrow_order.is_active, "Order already active");
        assert!(
            !self.escrows.get(&escrow_order.order_hash).is_some(),
            "Order already exists"
        );

        let mut order = escrow_order.clone();
        order.is_active = true;
        order.timelocks.deployed_at = env::block_timestamp();

        Promise::new(order.token_contract.clone())
            .function_call(
                "ft_transfer_from".as_bytes().to_vec(),
                format!(
                    r#"{{"sender_id":"{}","receiver_id":"{}","amount":"{}","memo":"Fusion escrow deposit"}}"#,
                    maker,
                    env::current_account_id(),
                    order.amount.0
                ).as_bytes().to_vec(),
                1,
                GAS_FOR_FT_TRANSFER,
            );

        self.escrows.insert(&order.order_hash, &order);
        self.escrow_count += 1;

        order.order_hash
    }

    pub fn withdraw(&mut self, secret: String, order_hash: [u8; 32]) {
        let taker = env::predecessor_account_id();
        let order = self
            .escrows
            .get(&order_hash)
            .expect("Order not found");

        assert_eq!(order.taker, taker, "Only taker can withdraw");
        assert!(order.is_active, "Order not active");

        assert!(
            order
                .timelocks
                .is_stage_time(Stage::SrcWithdrawal, TimeBoundKind::After),
            "Too early to withdraw"
        );
        assert!(
            order
                .timelocks
                .is_stage_time(Stage::SrcCancellation, TimeBoundKind::Before),
            "Too late to withdraw"
        );

        self.verify_secret(&secret, &order.secret_hash);
        self.transfer_to_taker(&order);
        self.deactivate_order(order_hash);
    }

    pub fn withdraw_to(&mut self, secret: String, target: AccountId, order_hash: [u8; 32]) {
        let taker = env::predecessor_account_id();
        let order = self
            .escrows
            .get(&order_hash)
            .expect("Order not found");

        assert_eq!(order.taker, taker, "Only taker can withdraw");
        assert!(order.is_active, "Order not active");

        assert!(
            order
                .timelocks
                .is_stage_time(Stage::SrcWithdrawal, TimeBoundKind::After),
            "Too early to withdraw"
        );
        assert!(
            order
                .timelocks
                .is_stage_time(Stage::SrcCancellation, TimeBoundKind::Before),
            "Too late to withdraw"
        );

        self.verify_secret(&secret, &order.secret_hash);
        self.transfer_to_target(&order, &target);
        self.deactivate_order(order_hash);
    }

    pub fn public_withdraw(&mut self, secret: String, order_hash: [u8; 32]) {
        let order = self
            .escrows
            .get(&order_hash)
            .expect("Order not found");

        assert!(order.is_active, "Order not active");

        assert!(
            order
                .timelocks
                .is_stage_time(Stage::SrcPublicWithdrawal, TimeBoundKind::After),
            "Too early for public withdrawal"
        );
        assert!(
            order
                .timelocks
                .is_stage_time(Stage::SrcCancellation, TimeBoundKind::Before),
            "Too late to withdraw"
        );

        self.verify_secret(&secret, &order.secret_hash);
        self.transfer_to_taker(&order);
        self.deactivate_order(order_hash);
    }

    pub fn cancel(&mut self, order_hash: [u8; 32]) {
        let maker = env::predecessor_account_id();
        let order = self
            .escrows
            .get(&order_hash)
            .expect("Order not found");

        assert_eq!(order.maker, maker, "Only maker can cancel");
        assert!(order.is_active, "Order not active");

        assert!(
            order
                .timelocks
                .is_stage_time(Stage::SrcCancellation, TimeBoundKind::After),
            "Too early to cancel"
        );

        self.transfer_to_maker(&order);
        self.deactivate_order(order_hash);
    }

    pub fn public_cancel(&mut self, order_hash: [u8; 32]) {
        let order = self
            .escrows
            .get(&order_hash)
            .expect("Order not found");

        assert!(order.is_active, "Order not active");

        assert!(
            order
                .timelocks
                .is_stage_time(Stage::SrcPublicCancellation, TimeBoundKind::After),
            "Too early for public cancellation"
        );

        self.transfer_to_maker(&order);
        self.deactivate_order(order_hash);
    }

    pub fn get_order(&self, order_hash: [u8; 32]) -> Option<EscrowOrder> {
        self.escrows.get(&order_hash)
    }

    pub fn get_escrow_count(&self) -> u64 {
        self.escrow_count
    }

    fn verify_secret(&self, secret: &str, expected_hash: &[u8; 32]) {
        let secret_bytes = secret.as_bytes();
        let mut hasher = Keccak256::new();
        hasher.update(secret_bytes);
        let computed_hash: [u8; 32] = hasher.finalize().into();
        
        assert_eq!(computed_hash, *expected_hash, "Invalid secret");
    }

    fn transfer_to_taker(&self, order: &EscrowOrder) {
        Promise::new(order.token_contract.clone())
            .function_call(
                "ft_transfer".as_bytes().to_vec(),
                format!(
                    r#"{{"receiver_id":"{}","amount":"{}","memo":"Fusion escrow withdrawal"}}"#,
                    order.taker,
                    order.amount.0
                ).as_bytes().to_vec(),
                1,
                GAS_FOR_FT_TRANSFER,
            );
    }

    fn transfer_to_target(&self, order: &EscrowOrder, target: &AccountId) {
        Promise::new(order.token_contract.clone())
            .function_call(
                "ft_transfer".as_bytes().to_vec(),
                format!(
                    r#"{{"receiver_id":"{}","amount":"{}","memo":"Fusion escrow withdrawal to target"}}"#,
                    target,
                    order.amount.0
                ).as_bytes().to_vec(),
                1,
                GAS_FOR_FT_TRANSFER,
            );
    }

    fn transfer_to_maker(&self, order: &EscrowOrder) {
        Promise::new(order.token_contract.clone())
            .function_call(
                "ft_transfer".as_bytes().to_vec(),
                format!(
                    r#"{{"receiver_id":"{}","amount":"{}","memo":"Fusion escrow cancellation"}}"#,
                    order.maker,
                    order.amount.0
                ).as_bytes().to_vec(),
                1,
                GAS_FOR_FT_TRANSFER,
            );
    }

    fn deactivate_order(&mut self, order_hash: [u8; 32]) {
        if let Some(mut order) = self.escrows.get(&order_hash) {
            order.is_active = false;
            self.escrows.insert(&order_hash, &order);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id)
            .build()
    }

    #[test]
    fn test_deposit() {
        let context = get_context(accounts(1));
        testing_env!(context);
        
        let mut contract = FusionEscrow::new();
        
        let timelocks = Timelocks {
            src_withdrawal: 1000,
            src_public_withdrawal: 2000,
            src_cancellation: 3000,
            src_public_cancellation: 4000,
            dst_withdrawal: 1000,
            dst_public_withdrawal: 2000,
            dst_cancellation: 3000,
            deployed_at: 0,
        };

        let order = EscrowOrder {
            order_hash: [1; 32],
            secret_hash: [2; 32],
            maker: accounts(1),
            taker: accounts(2),
            token_contract: accounts(3),
            amount: U128(1000),
            safety_deposit: U128(100),
            timelocks,
            is_active: false,
        };

        let result = contract.deposit(order);
        assert_eq!(result, [1; 32]);
        assert_eq!(contract.get_escrow_count(), 1);
    }

    #[test]
    fn test_secret_verification() {
        let contract = FusionEscrow::new();
        let secret = "test_secret";
        let secret_bytes = secret.as_bytes();
        let mut hasher = Keccak256::new();
        hasher.update(secret_bytes);
        let expected_hash: [u8; 32] = hasher.finalize().into();

        contract.verify_secret(secret, &expected_hash);
    }

    #[test]
    #[should_panic(expected = "Invalid secret")]
    fn test_invalid_secret() {
        let contract = FusionEscrow::new();
        let wrong_hash = [0; 32];
        contract.verify_secret("test_secret", &wrong_hash);
    }
}