use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise};
use escrow::{Immutables, FusionOrder, verify_secret};

const GAS_FOR_FT_TRANSFER: u64 = 10_000_000_000_000;
const GAS_FOR_CROSS_CONTRACT: u64 = 20_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CrossChainOrder {
    pub src_order: FusionOrder,
    pub dst_order: FusionOrder,
    pub src_escrow: Option<AccountId>,
    pub dst_escrow: Option<AccountId>,
    pub secret_hash: [u8; 32],
    pub resolver: AccountId,
    pub is_completed: bool,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FusionResolver {
    cross_chain_orders: UnorderedMap<[u8; 32], CrossChainOrder>,
    escrow_factory: AccountId,
    fusion_order_contract: AccountId,
    resolver_operators: UnorderedMap<AccountId, bool>,
}

#[near_bindgen]
impl FusionResolver {
    #[init]
    pub fn new(escrow_factory: AccountId, fusion_order_contract: AccountId) -> Self {
        Self {
            cross_chain_orders: UnorderedMap::new(b"c"),
            escrow_factory,
            fusion_order_contract,
            resolver_operators: UnorderedMap::new(b"o"),
        }
    }

    pub fn initiate_cross_chain_swap(
        &mut self,
        src_order: FusionOrder,
        dst_order: FusionOrder,
        secret_hash: [u8; 32],
    ) -> [u8; 32] {
        let resolver = env::predecessor_account_id();
        assert!(
            self.resolver_operators.get(&resolver).unwrap_or(false),
            "Not authorized resolver"
        );
        
        let order_hash = self._compute_cross_chain_hash(&src_order, &dst_order, &secret_hash);
        
        assert!(
            !self.cross_chain_orders.get(&order_hash).is_some(),
            "Order already exists"
        );
        
        let cross_chain_order = CrossChainOrder {
            src_order,
            dst_order,
            src_escrow: None,
            dst_escrow: None,
            secret_hash,
            resolver,
            is_completed: false,
        };
        
        self.cross_chain_orders.insert(&order_hash, &cross_chain_order);
        order_hash
    }

    pub fn deploy_src_escrow(&mut self, order_hash: [u8; 32], immutables: Immutables) -> Promise {
        let mut order = self.cross_chain_orders.get(&order_hash).expect("Order not found");
        let resolver = env::predecessor_account_id();
        
        assert_eq!(order.resolver, resolver, "Only order resolver can deploy");
        assert!(order.src_escrow.is_none(), "Src escrow already deployed");
        
        Promise::new(self.escrow_factory.clone()).function_call(
            "create_src_escrow".as_bytes().to_vec(),
            serde_json::to_vec(&immutables).unwrap(),
            env::attached_deposit(),
            GAS_FOR_CROSS_CONTRACT,
        ).then(
            Promise::new(env::current_account_id()).function_call(
                "on_src_escrow_created".as_bytes().to_vec(),
                serde_json::json!({
                    "order_hash": order_hash,
                }).to_string().as_bytes().to_vec(),
                0,
                GAS_FOR_CROSS_CONTRACT,
            )
        )
    }

    pub fn deploy_dst_escrow(
        &mut self,
        order_hash: [u8; 32],
        immutables: Immutables,
        src_cancellation_timestamp: u64,
    ) -> Promise {
        let mut order = self.cross_chain_orders.get(&order_hash).expect("Order not found");
        let taker = env::predecessor_account_id();
        
        assert_eq!(immutables.taker, taker, "Only taker can deploy dst");
        assert!(order.dst_escrow.is_none(), "Dst escrow already deployed");
        
        Promise::new(self.escrow_factory.clone()).function_call(
            "create_dst_escrow".as_bytes().to_vec(),
            serde_json::json!({
                "immutables": immutables,
                "src_cancellation_timestamp": src_cancellation_timestamp,
            }).to_string().as_bytes().to_vec(),
            env::attached_deposit(),
            GAS_FOR_CROSS_CONTRACT,
        ).then(
            Promise::new(env::current_account_id()).function_call(
                "on_dst_escrow_created".as_bytes().to_vec(),
                serde_json::json!({
                    "order_hash": order_hash,
                }).to_string().as_bytes().to_vec(),
                0,
                GAS_FOR_CROSS_CONTRACT,
            )
        )
    }

    pub fn execute_cross_chain_fill(
        &mut self,
        order_hash: [u8; 32],
        secret: String,
        making_amount: U128,
        taking_amount: U128,
    ) {
        let order = self.cross_chain_orders.get(&order_hash).expect("Order not found");
        let resolver = env::predecessor_account_id();
        
        assert_eq!(order.resolver, resolver, "Only resolver can execute");
        assert!(!order.is_completed, "Order already completed");
        assert!(verify_secret(&secret, &order.secret_hash), "Invalid secret");
        
        if let (Some(src_escrow), Some(dst_escrow)) = (&order.src_escrow, &order.dst_escrow) {
            let immutables = self._create_immutables_from_orders(&order.src_order, &order.dst_order, &order.secret_hash);
            
            Promise::new(src_escrow.clone()).function_call(
                "withdraw".as_bytes().to_vec(),
                serde_json::json!({
                    "secret": secret.clone(),
                    "immutables": immutables.clone(),
                }).to_string().as_bytes().to_vec(),
                0,
                GAS_FOR_CROSS_CONTRACT,
            );
            
            Promise::new(dst_escrow.clone()).function_call(
                "withdraw".as_bytes().to_vec(),
                serde_json::json!({
                    "secret": secret,
                    "immutables": immutables,
                }).to_string().as_bytes().to_vec(),
                0,
                GAS_FOR_CROSS_CONTRACT,
            );
            
            Promise::new(self.fusion_order_contract.clone()).function_call(
                "fill_order".as_bytes().to_vec(),
                serde_json::json!({
                    "order_hash": order.src_order.hash(),
                    "making_amount": making_amount,
                    "taking_amount": taking_amount,
                    "resolver_fee": null,
                }).to_string().as_bytes().to_vec(),
                0,
                GAS_FOR_CROSS_CONTRACT,
            );
        } else {
            panic!("Escrows not deployed");
        }
    }

    pub fn add_operator(&mut self, operator: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only contract can add operators"
        );
        self.resolver_operators.insert(&operator, &true);
    }

    pub fn remove_operator(&mut self, operator: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only contract can remove operators"
        );
        self.resolver_operators.remove(&operator);
    }

    #[private]
    pub fn on_src_escrow_created(&mut self, order_hash: [u8; 32], escrow_address: AccountId) {
        let mut order = self.cross_chain_orders.get(&order_hash).expect("Order not found");
        order.src_escrow = Some(escrow_address);
        self.cross_chain_orders.insert(&order_hash, &order);
    }

    #[private]
    pub fn on_dst_escrow_created(&mut self, order_hash: [u8; 32], escrow_address: AccountId) {
        let mut order = self.cross_chain_orders.get(&order_hash).expect("Order not found");
        order.dst_escrow = Some(escrow_address);
        self.cross_chain_orders.insert(&order_hash, &order);
    }

    fn _compute_cross_chain_hash(
        &self,
        src_order: &FusionOrder,
        dst_order: &FusionOrder,
        secret_hash: &[u8; 32],
    ) -> [u8; 32] {
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        hasher.update(&src_order.hash());
        hasher.update(&dst_order.hash());
        hasher.update(secret_hash);
        hasher.finalize().into()
    }

    fn _create_immutables_from_orders(
        &self,
        src_order: &FusionOrder,
        dst_order: &FusionOrder,
        secret_hash: &[u8; 32],
    ) -> Immutables {
        use escrow::Timelocks;
        
        Immutables {
            order_hash: src_order.hash(),
            hashlock: *secret_hash,
            maker: src_order.maker.clone(),
            taker: dst_order.maker.clone(),
            token: src_order.maker_asset.clone(),
            amount: src_order.making_amount.clone(),
            safety_deposit: U128(0),
            timelocks: Timelocks {
                src_withdrawal: 3600,
                src_public_withdrawal: 7200,
                src_cancellation: 10800,
                src_public_cancellation: 14400,
                dst_withdrawal: 1800,
                dst_public_withdrawal: 3600,
                dst_cancellation: 7200,
                deployed_at: env::block_timestamp(),
            },
        }
    }
}