export const CONTRACT_CONFIG = {
  networkId: "testnet",
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://explorer.testnet.near.org",
  contracts: {
    fusionOrder: "fusion-order.testnet",
    fusionResolver: "fusion-resolver.testnet",
    escrowFactory: "escrow-factory.testnet",
    // Token contracts for testing
    wNEAR: "wrap.testnet",
    USDC: "usdc.fakes.testnet",
    USDT: "usdt.fakes.testnet",
  },
};

export const TOKENS = [
  {
    id: CONTRACT_CONFIG.contracts.wNEAR,
    symbol: "wNEAR",
    name: "Wrapped NEAR",
    decimals: 24,
    icon: "https://assets.coingecko.com/coins/images/10365/small/near.jpg",
  },
  {
    id: CONTRACT_CONFIG.contracts.USDC,
    symbol: "USDC",
    name: "USD Coin",
    decimals: 6,
    icon: "https://assets.coingecko.com/coins/images/6319/small/USD_Coin_icon.png",
  },
  {
    id: CONTRACT_CONFIG.contracts.USDT,
    symbol: "USDT",
    name: "Tether USD",
    decimals: 6,
    icon: "https://assets.coingecko.com/coins/images/325/small/Tether.png",
  },
];

export const TIMELOCK_STAGES = {
  srcWithdrawal: 3600, // 1 hour
  srcPublicWithdrawal: 7200, // 2 hours
  srcCancellation: 10800, // 3 hours
  srcPublicCancellation: 14400, // 4 hours
  dstWithdrawal: 1800, // 30 minutes
  dstPublicWithdrawal: 3600, // 1 hour
  dstCancellation: 7200, // 2 hours
};

// Mock contract calls for demonstration
export const mockContractCalls = {
  create_order: async (order: any, auction: any) => {
    console.log('Mock: Creating order', { order, auction });
    return {
      order_hash: `0x${Math.random().toString(16).slice(2, 10)}`,
      status: 'created'
    };
  },
  
  get_user_orders: async (accountId: string) => {
    console.log('Mock: Getting orders for', accountId);
    return [
      {
        order_hash: '0xabcd1234',
        maker_asset: CONTRACT_CONFIG.contracts.wNEAR,
        taker_asset: CONTRACT_CONFIG.contracts.USDC,
        making_amount: '1000000000000000000000000',
        taking_amount: '1000000',
        status: 'active',
        filled_amount: '0'
      },
      {
        order_hash: '0xefgh5678',
        maker_asset: CONTRACT_CONFIG.contracts.USDC,
        taker_asset: CONTRACT_CONFIG.contracts.wNEAR,
        making_amount: '5000000',
        taking_amount: '5000000000000000000000000',
        status: 'active',
        filled_amount: '1500000'
      }
    ];
  }
};