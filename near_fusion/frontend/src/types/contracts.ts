export interface FusionOrder {
  order_hash: string;
  maker: string;
  resolver?: string;
  maker_asset: string;
  taker_asset: string;
  making_amount: string;
  taking_amount: string;
  maker_traits: number;
  salt: string;
}

export interface AuctionDetails {
  start_time: number;
  duration: number;
  initial_rate_bump: number;
  points: AuctionPoint[];
}

export interface AuctionPoint {
  delay: number;
  coefficient: number;
}

export interface OrderState {
  order: FusionOrder;
  filled_making_amount: string;
  filled_taking_amount: string;
  is_cancelled: boolean;
  auction?: AuctionDetails;
}

export interface Immutables {
  order_hash: string;
  hashlock: string;
  maker: string;
  taker: string;
  token: string;
  amount: string;
  safety_deposit: string;
  timelocks: Timelocks;
}

export interface Timelocks {
  src_withdrawal: number;
  src_public_withdrawal: number;
  src_cancellation: number;
  src_public_cancellation: number;
  dst_withdrawal: number;
  dst_public_withdrawal: number;
  dst_cancellation: number;
  deployed_at: number;
}

export interface Token {
  id: string;
  symbol: string;
  name: string;
  decimals: number;
  icon: string;
}