use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise, Balance};
use escrow::{Immutables, Stage, DataKey};

#[cfg(test)]
mod test;

const GAS_FOR_FT_TRANSFER: u64 = 10_000_000_000_000;
const GAS_FOR_DEPLOY: u64 = 50_000_000_000_000;
const ESCROW_DEPOSIT: Balance = 5_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct EscrowFactory {
    escrow_src_code: Vec<u8>,
    escrow_dst_code: Vec<u8>,
    deployments: UnorderedMap<[u8; 32], AccountId>,
}

#[near_bindgen]
impl EscrowFactory {
    #[init]
    pub fn new(escrow_src_code: Vec<u8>, escrow_dst_code: Vec<u8>) -> Self {
        Self {
            escrow_src_code,
            escrow_dst_code,
            deployments: UnorderedMap::new(b"d"),
        }
    }

    #[payable]
    pub fn create_src_escrow(&mut self, immutables: Immutables) -> AccountId {
        let factory = env::current_account_id();
        let salt = immutables.hash();
        
        let escrow_account_id = self._get_escrow_account_id(&salt, "src");
        
        if self.deployments.get(&salt).is_some() {
            panic!("Escrow already deployed");
        }
        
        Promise::new(escrow_account_id.clone())
            .create_account()
            .transfer(ESCROW_DEPOSIT)
            .deploy_contract(self.escrow_src_code.clone())
            .function_call(
                "new".as_bytes().to_vec(),
                format!(r#"{{"factory":"{}"}}"#, factory).as_bytes().to_vec(),
                0,
                GAS_FOR_DEPLOY,
            );
        
        Promise::new(immutables.token.clone()).function_call(
            "ft_transfer_from".as_bytes().to_vec(),
            format!(
                r#"{{"sender_id":"{}","receiver_id":"{}","amount":"{}","memo":"Escrow deposit"}}"#,
                immutables.maker,
                escrow_account_id,
                immutables.amount.0
            )
            .as_bytes()
            .to_vec(),
            1,
            GAS_FOR_FT_TRANSFER,
        );
        
        self.deployments.insert(&salt, &escrow_account_id);
        
        escrow_account_id
    }

    #[payable]
    pub fn create_dst_escrow(&mut self, immutables: Immutables, src_cancellation_timestamp: u64) -> AccountId {
        let taker = env::predecessor_account_id();
        assert_eq!(immutables.taker, taker, "Only taker can create dst escrow");
        
        if immutables.timelocks.get_stage_time(Stage::DstCancellation) > src_cancellation_timestamp {
            panic!("Invalid creation time");
        }
        
        let factory = env::current_account_id();
        let salt = immutables.hash();
        
        let escrow_account_id = self._get_escrow_account_id(&salt, "dst");
        
        if self.deployments.get(&salt).is_some() {
            panic!("Escrow already deployed");
        }
        
        Promise::new(escrow_account_id.clone())
            .create_account()
            .transfer(ESCROW_DEPOSIT)
            .deploy_contract(self.escrow_dst_code.clone())
            .function_call(
                "new".as_bytes().to_vec(),
                format!(r#"{{"factory":"{}"}}"#, factory).as_bytes().to_vec(),
                0,
                GAS_FOR_DEPLOY,
            );
        
        Promise::new(immutables.token.clone()).function_call(
            "ft_transfer".as_bytes().to_vec(),
            format!(
                r#"{{"receiver_id":"{}","amount":"{}","memo":"Escrow deposit"}}"#,
                escrow_account_id,
                immutables.amount.0
            )
            .as_bytes()
            .to_vec(),
            1,
            GAS_FOR_FT_TRANSFER,
        );
        
        self.deployments.insert(&salt, &escrow_account_id);
        
        escrow_account_id
    }

    pub fn get_deployment(&self, salt: [u8; 32]) -> Option<AccountId> {
        self.deployments.get(&salt)
    }

    pub fn _get_escrow_account_id(&self, salt: &[u8; 32], prefix: &str) -> AccountId {
        let hex_salt = hex::encode(salt);
        format!("{}-{}.{}", prefix, &hex_salt[..8], env::current_account_id())
            .parse()
            .unwrap()
    }
}