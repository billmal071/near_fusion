use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};
use sha3::{Digest, Keccak256};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum DataKey {
    Factory,
    EscrowSrcHash,
    EscrowDstHash,
}

#[derive(Clone)]
pub enum TimeBoundKind {
    Before,
    After,
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
    pub fn get_stage_time(&self, stage: Stage) -> u64 {
        match stage {
            Stage::SrcWithdrawal => self.deployed_at + self.src_withdrawal,
            Stage::SrcPublicWithdrawal => self.deployed_at + self.src_public_withdrawal,
            Stage::SrcCancellation => self.deployed_at + self.src_cancellation,
            Stage::SrcPublicCancellation => self.deployed_at + self.src_public_cancellation,
            Stage::DstWithdrawal => self.deployed_at + self.dst_withdrawal,
            Stage::DstPublicWithdrawal => self.deployed_at + self.dst_public_withdrawal,
            Stage::DstCancellation => self.deployed_at + self.dst_cancellation,
        }
    }

    pub fn is_stage_time(&self, stage: Stage, time_bound_kind: TimeBoundKind) -> bool {
        let time_bound_timestamp = self.get_stage_time(stage);
        let current_timestamp = env::block_timestamp();
        match time_bound_kind {
            TimeBoundKind::Before => current_timestamp < time_bound_timestamp,
            TimeBoundKind::After => current_timestamp >= time_bound_timestamp,
        }
    }

    pub fn set_deployed_at(&mut self, timestamp: u64) {
        self.deployed_at = timestamp;
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Immutables {
    pub order_hash: [u8; 32],
    pub hashlock: [u8; 32],
    pub maker: AccountId,
    pub taker: AccountId,
    pub token: AccountId,
    pub amount: U128,
    pub safety_deposit: U128,
    pub timelocks: Timelocks,
}

impl Immutables {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(&self.order_hash);
        hasher.update(&self.hashlock);
        hasher.update(self.maker.as_bytes());
        hasher.update(self.taker.as_bytes());
        hasher.update(self.token.as_bytes());
        hasher.update(&self.amount.0.to_le_bytes());
        hasher.update(&self.safety_deposit.0.to_le_bytes());
        hasher.update(&self.timelocks.src_withdrawal.to_le_bytes());
        hasher.update(&self.timelocks.src_public_withdrawal.to_le_bytes());
        hasher.update(&self.timelocks.src_cancellation.to_le_bytes());
        hasher.update(&self.timelocks.src_public_cancellation.to_le_bytes());
        hasher.update(&self.timelocks.dst_withdrawal.to_le_bytes());
        hasher.update(&self.timelocks.dst_public_withdrawal.to_le_bytes());
        hasher.update(&self.timelocks.dst_cancellation.to_le_bytes());
        hasher.update(&self.timelocks.deployed_at.to_le_bytes());
        hasher.finalize().into()
    }
}

pub fn verify_secret(secret: &str, expected_hash: &[u8; 32]) -> bool {
    let mut hasher = Keccak256::new();
    hasher.update(secret.as_bytes());
    let computed_hash: [u8; 32] = hasher.finalize().into();
    computed_hash == *expected_hash
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FusionOrder {
    pub order_hash: [u8; 32],
    pub maker: AccountId,
    pub resolver: Option<AccountId>,
    pub maker_asset: AccountId,
    pub taker_asset: AccountId,
    pub making_amount: U128,
    pub taking_amount: U128,
    pub maker_traits: u64,
    pub salt: [u8; 32],
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AuctionDetails {
    pub start_time: u64,
    pub duration: u64,
    pub initial_rate_bump: u32,
    pub points: Vec<AuctionPoint>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AuctionPoint {
    pub delay: u32,
    pub coefficient: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ResolverFee {
    pub receiver: AccountId,
    pub amount: U128,
}

impl FusionOrder {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(&self.order_hash);
        hasher.update(self.maker.as_bytes());
        if let Some(resolver) = &self.resolver {
            hasher.update(resolver.as_bytes());
        }
        hasher.update(self.maker_asset.as_bytes());
        hasher.update(self.taker_asset.as_bytes());
        hasher.update(&self.making_amount.0.to_le_bytes());
        hasher.update(&self.taking_amount.0.to_le_bytes());
        hasher.update(&self.maker_traits.to_le_bytes());
        hasher.update(&self.salt);
        hasher.finalize().into()
    }

    pub fn is_valid_partial_fill(&self, filled_making_amount: u128, filled_taking_amount: u128) -> bool {
        if filled_making_amount == 0 || filled_taking_amount == 0 {
            return false;
        }
        
        let expected_ratio = self.taking_amount.0 * filled_making_amount;
        let actual_ratio = self.making_amount.0 * filled_taking_amount;
        
        expected_ratio == actual_ratio
    }
}

impl AuctionDetails {
    pub fn get_rate_bump(&self, current_time: u64) -> u32 {
        if current_time < self.start_time {
            return self.initial_rate_bump;
        }
        
        let elapsed = current_time - self.start_time;
        if elapsed >= self.duration {
            return 0;
        }
        
        let mut current_bump = self.initial_rate_bump;
        
        for point in &self.points {
            if elapsed >= point.delay as u64 {
                current_bump = current_bump.saturating_mul(point.coefficient) / 1_000_000;
            }
        }
        
        current_bump
    }
}