use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise};
use escrow::{FusionOrder, AuctionDetails, ResolverFee};

#[cfg(test)]
mod tests;

const GAS_FOR_FT_TRANSFER: u64 = 10_000_000_000_000;
const BASIS_POINTS: u32 = 10_000;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OrderState {
    pub order: FusionOrder,
    pub filled_making_amount: u128,
    pub filled_taking_amount: u128,
    pub is_cancelled: bool,
    pub auction: Option<AuctionDetails>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FusionOrderContract {
    orders: UnorderedMap<[u8; 32], OrderState>,
    resolver_whitelist: UnorderedMap<AccountId, bool>,
    protocol_fee_receiver: AccountId,
    protocol_fee_basis_points: u32,
}

#[near_bindgen]
impl FusionOrderContract {
    #[init]
    pub fn new(protocol_fee_receiver: AccountId, protocol_fee_basis_points: u32) -> Self {
        assert!(protocol_fee_basis_points <= BASIS_POINTS, "Invalid fee");
        Self {
            orders: UnorderedMap::new(b"o"),
            resolver_whitelist: UnorderedMap::new(b"r"),
            protocol_fee_receiver,
            protocol_fee_basis_points,
        }
    }

    pub fn create_order(&mut self, order: FusionOrder, auction: Option<AuctionDetails>) -> [u8; 32] {
        let maker = env::predecessor_account_id();
        assert_eq!(order.maker, maker, "Only maker can create order");
        
        let order_hash = order.hash();
        assert!(!self.orders.get(&order_hash).is_some(), "Order exists");
        
        let order_state = OrderState {
            order,
            filled_making_amount: 0,
            filled_taking_amount: 0,
            is_cancelled: false,
            auction,
        };
        
        self.orders.insert(&order_hash, &order_state);
        order_hash
    }

    pub fn fill_order(
        &mut self,
        order_hash: [u8; 32],
        making_amount: U128,
        taking_amount: U128,
        resolver_fee: Option<ResolverFee>,
    ) {
        let taker = env::predecessor_account_id();
        let mut order_state = self.orders.get(&order_hash).expect("Order not found");
        
        assert!(!order_state.is_cancelled, "Order cancelled");
        
        let making_amount = making_amount.0;
        let taking_amount = taking_amount.0;
        
        assert!(
            order_state.order.is_valid_partial_fill(making_amount, taking_amount),
            "Invalid fill amounts"
        );
        
        let remaining_making = order_state.order.making_amount.0 - order_state.filled_making_amount;
        let remaining_taking = order_state.order.taking_amount.0 - order_state.filled_taking_amount;
        
        assert!(making_amount <= remaining_making, "Exceeds available");
        assert!(taking_amount <= remaining_taking, "Exceeds required");
        
        let mut actual_taking_amount = taking_amount;
        
        if let Some(auction) = &order_state.auction {
            let rate_bump = auction.get_rate_bump(env::block_timestamp());
            actual_taking_amount = taking_amount + (taking_amount * rate_bump as u128) / BASIS_POINTS as u128;
        }
        
        if let Some(resolver_fee) = resolver_fee {
            if let Some(resolver) = &order_state.order.resolver {
                assert_eq!(resolver_fee.receiver, *resolver, "Invalid resolver");
                assert!(
                    self.resolver_whitelist.get(resolver).unwrap_or(false),
                    "Resolver not whitelisted"
                );
                actual_taking_amount += resolver_fee.amount.0;
            }
        }
        
        let protocol_fee = (making_amount * self.protocol_fee_basis_points as u128) / BASIS_POINTS as u128;
        let maker_receives = actual_taking_amount;
        let taker_receives = making_amount - protocol_fee;
        
        Promise::new(order_state.order.taker_asset.clone()).function_call(
            "ft_transfer_from".as_bytes().to_vec(),
            format!(
                r#"{{"sender_id":"{}","receiver_id":"{}","amount":"{}","memo":"Fusion fill"}}"#,
                taker,
                order_state.order.maker,
                maker_receives
            )
            .as_bytes()
            .to_vec(),
            1,
            GAS_FOR_FT_TRANSFER,
        );
        
        Promise::new(order_state.order.maker_asset.clone()).function_call(
            "ft_transfer_from".as_bytes().to_vec(),
            format!(
                r#"{{"sender_id":"{}","receiver_id":"{}","amount":"{}","memo":"Fusion fill"}}"#,
                order_state.order.maker,
                taker,
                taker_receives
            )
            .as_bytes()
            .to_vec(),
            1,
            GAS_FOR_FT_TRANSFER,
        );
        
        if protocol_fee > 0 {
            Promise::new(order_state.order.maker_asset.clone()).function_call(
                "ft_transfer_from".as_bytes().to_vec(),
                format!(
                    r#"{{"sender_id":"{}","receiver_id":"{}","amount":"{}","memo":"Protocol fee"}}"#,
                    order_state.order.maker,
                    self.protocol_fee_receiver,
                    protocol_fee
                )
                .as_bytes()
                .to_vec(),
                1,
                GAS_FOR_FT_TRANSFER,
            );
        }
        
        order_state.filled_making_amount += making_amount;
        order_state.filled_taking_amount += taking_amount;
        
        self.orders.insert(&order_hash, &order_state);
    }

    pub fn cancel_order(&mut self, order_hash: [u8; 32]) {
        let maker = env::predecessor_account_id();
        let mut order_state = self.orders.get(&order_hash).expect("Order not found");
        
        assert_eq!(order_state.order.maker, maker, "Only maker can cancel");
        assert!(!order_state.is_cancelled, "Already cancelled");
        
        order_state.is_cancelled = true;
        self.orders.insert(&order_hash, &order_state);
    }

    pub fn add_resolver(&mut self, resolver: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only contract can add resolvers"
        );
        self.resolver_whitelist.insert(&resolver, &true);
    }

    pub fn remove_resolver(&mut self, resolver: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only contract can remove resolvers"
        );
        self.resolver_whitelist.remove(&resolver);
    }

    pub fn get_order(&self, order_hash: [u8; 32]) -> Option<OrderState> {
        self.orders.get(&order_hash)
    }

    pub fn get_remaining_amounts(&self, order_hash: [u8; 32]) -> (U128, U128) {
        if let Some(order_state) = self.orders.get(&order_hash) {
            let remaining_making = order_state.order.making_amount.0 - order_state.filled_making_amount;
            let remaining_taking = order_state.order.taking_amount.0 - order_state.filled_taking_amount;
            (U128(remaining_making), U128(remaining_taking))
        } else {
            (U128(0), U128(0))
        }
    }
}