// Mock deployed contract addresses for demonstration
export const DEPLOYED_CONTRACTS = {
  networkId: 'testnet' as const,
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
  contracts: {
    // NEAR Contracts (Mock addresses for demo)
    factory: 'factory.demo172092.testnet',
    fusionOrder: 'fusion-order.demo172092.testnet',
    fusionResolver: 'fusion-resolver.demo172092.testnet',
    
    // Token addresses (real testnet tokens)
    wNEAR: 'wrap.testnet',
    USDC: 'usdc.fakes.testnet',
    USDT: 'usdt.fakes.testnet',
  },
  
  // Ethereum contracts (would be deployed on Ethereum)
  ethereum: {
    fusionRouter: '0x1234567890123456789012345678901234567890',
    htlcFactory: '0x2345678901234567890123456789012345678901',
    resolver: '0x3456789012345678901234567890123456789012',
  }
};

// Example of how cross-chain orders would work
export const CROSS_CHAIN_FLOW = {
  1: "User creates order on Ethereum to swap USDC for wNEAR",
  2: "Order is posted to resolver network",
  3: "Matching order found on NEAR (wNEAR for USDC)",
  4: "HTLC contracts created on both chains with same secret hash",
  5: "Resolver reveals secret to claim NEAR tokens",
  6: "User uses same secret to claim Ethereum tokens",
  7: "Atomic swap completed!"
};
