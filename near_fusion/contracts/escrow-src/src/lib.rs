use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise};
use escrow::{verify_secret, Immutables, Stage, TimeBoundKind, DataKey};

const GAS_FOR_FT_TRANSFER: u64 = 5_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct EscrowSrc {
    factory: AccountId,
}

#[near_bindgen]
impl EscrowSrc {
    #[init]
    pub fn new(factory: AccountId) -> Self {
        Self { factory }
    }

    pub fn withdraw(&mut self, secret: String, immutables: Immutables) {
        let taker = env::predecessor_account_id();
        assert_eq!(immutables.taker, taker, "Only taker can withdraw");
        
        assert!(
            !immutables.timelocks.is_stage_time(Stage::SrcWithdrawal, TimeBoundKind::Before),
            "Too early"
        );
        assert!(
            !immutables.timelocks.is_stage_time(Stage::SrcCancellation, TimeBoundKind::After),
            "Too late"
        );
        
        self._withdraw_to(secret, immutables.taker.clone(), immutables);
    }

    pub fn withdraw_to(&mut self, secret: String, target: AccountId, immutables: Immutables) {
        let taker = env::predecessor_account_id();
        assert_eq!(immutables.taker, taker, "Only taker can withdraw");
        
        assert!(
            !immutables.timelocks.is_stage_time(Stage::SrcWithdrawal, TimeBoundKind::Before),
            "Too early"
        );
        assert!(
            !immutables.timelocks.is_stage_time(Stage::SrcCancellation, TimeBoundKind::After),
            "Too late"
        );
        
        self._withdraw_to(secret, target, immutables);
    }

    pub fn public_withdraw(&mut self, secret: String, immutables: Immutables) {
        assert!(
            !immutables.timelocks.is_stage_time(Stage::SrcPublicWithdrawal, TimeBoundKind::Before),
            "Too early"
        );
        assert!(
            !immutables.timelocks.is_stage_time(Stage::SrcCancellation, TimeBoundKind::After),
            "Too late"
        );
        
        self._withdraw_to(secret, immutables.taker.clone(), immutables);
    }

    pub fn cancel(&mut self, immutables: Immutables) {
        let taker = env::predecessor_account_id();
        assert_eq!(immutables.taker, taker, "Only taker can cancel");
        
        assert!(
            !immutables.timelocks.is_stage_time(Stage::SrcCancellation, TimeBoundKind::Before),
            "Too early"
        );
        
        self._transfer_to(&immutables.token, &immutables.maker, immutables.amount.0);
    }

    pub fn public_cancel(&mut self, immutables: Immutables) {
        assert!(
            !immutables.timelocks.is_stage_time(Stage::SrcPublicCancellation, TimeBoundKind::Before),
            "Too early"
        );
        
        self._transfer_to(&immutables.token, &immutables.maker, immutables.amount.0);
    }

    fn _withdraw_to(&mut self, secret: String, target: AccountId, immutables: Immutables) {
        assert!(
            verify_secret(&secret, &immutables.hashlock),
            "Invalid secret"
        );
        
        self._transfer_to(&immutables.token, &target, immutables.amount.0);
    }

    fn _transfer_to(&self, token: &AccountId, recipient: &AccountId, amount: u128) {
        Promise::new(token.clone()).function_call(
            "ft_transfer".as_bytes().to_vec(),
            format!(
                r#"{{"receiver_id":"{}","amount":"{}","memo":"Escrow transfer"}}"#,
                recipient, amount
            )
            .as_bytes()
            .to_vec(),
            1,
            GAS_FOR_FT_TRANSFER,
        );
    }
}