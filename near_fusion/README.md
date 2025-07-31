# NEAR Fusion+ Implementation

This is the NEAR Protocol implementation of the 1inch Fusion+ protocol, featuring cross-chain atomic swaps, limit orders with Dutch auction support, and resolver-based execution.

## Structure

```
near_fusion/
├── contracts/
│   ├── escrow-src/         # Source chain escrow contract
│   ├── escrow-dst/         # Destination chain escrow contract
│   ├── escrow-factory/     # Factory contract for deploying escrows
│   ├── fusion-order/       # Fusion+ limit order contract
│   └── fusion-resolver/    # Cross-chain resolver contract
├── shared/
│   └── escrow/             # Shared types and utilities
├── Cargo.toml              # Workspace configuration
└── Makefile                # Build automation
```

## Contracts

### Core HTLC Contracts

#### escrow-src
- Handles deposits on the source chain
- Manages withdrawals with secret verification
- Supports time-based stages for withdrawals and cancellations

#### escrow-dst
- Manages funds on the destination chain
- Handles withdrawals to maker upon secret reveal
- Supports cancellation back to taker after timeout

#### escrow-factory
- Deploys escrow contracts with deterministic addresses
- Manages contract code storage
- Handles initial fund transfers

### Fusion+ Contracts

#### fusion-order
- Implements 1inch Fusion+ limit order protocol
- Supports partial fills with proper ratio validation
- Dutch auction mechanism for dynamic pricing
- Resolver whitelisting and fee distribution
- Protocol fee collection

#### fusion-resolver
- Manages cross-chain order execution
- Coordinates HTLC escrows with Fusion orders
- Operator-based permission system
- Automated secret revelation and fund settlement

## Key Features

### 1inch Fusion+ Protocol
- **Limit Orders**: Gasless limit orders with maker/taker model
- **Partial Fills**: Orders can be filled partially while maintaining price ratios
- **Dutch Auction**: Time-based price improvement mechanism
- **Resolver Network**: Specialized executors compete to fill orders

### Cross-Chain Support
- **Atomic Swaps**: HTLC-based cross-chain transfers
- **Time-locked Stages**: Multiple withdrawal/cancellation windows
- **Secret Hash Verification**: Keccak256-based security

### Security Features
- **Whitelisted Resolvers**: Only authorized resolvers can execute
- **Time-based Protection**: Multiple stages prevent griefing
- **Deterministic Addresses**: Factory pattern ensures predictable deployments

## Building

```bash
# Build all contracts
make build

# Run tests
make test

# Format code
make fmt

# Clean build artifacts
make clean
```

## Usage Example

### Creating a Fusion+ Order
```rust
let order = FusionOrder {
    maker: "alice.near",
    maker_asset: "usdc.near",
    taker_asset: "wnear.near",
    making_amount: U128(1000_000000), // 1000 USDC
    taking_amount: U128(10_000000000000000000), // 10 NEAR
    // ... other fields
};

let auction = AuctionDetails {
    start_time: env::block_timestamp(),
    duration: 3600, // 1 hour
    initial_rate_bump: 200, // 2% initial bonus
    points: vec![
        AuctionPoint { delay: 900, coefficient: 500000 }, // 50% after 15 min
    ],
};

fusion_order_contract.create_order(order, Some(auction));
```

### Cross-Chain Execution Flow
1. Maker creates Fusion+ order on source chain
2. Resolver identifies arbitrage opportunity
3. Resolver deploys HTLC escrows on both chains
4. Taker fills order on destination chain
5. Resolver reveals secret to complete atomic swap

## Testing

Run the comprehensive test suite:
```bash
cargo test --workspace
```

## Frontend UI

A React-based testing interface is available in the `frontend/` directory:

```bash
cd frontend
npm install
npm run dev
```

The UI provides:
- Wallet connection
- Order creation with Dutch auction support
- Order management (view/cancel)
- Resolver dashboard (coming soon)

See [frontend/README.md](frontend/README.md) for detailed setup instructions.