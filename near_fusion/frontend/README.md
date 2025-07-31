# NEAR Fusion+ Frontend

A React-based UI for testing the NEAR Fusion+ cross-chain swap protocol.

## Features

- **NEAR Wallet Integration**: Connect with NEAR testnet wallets
- **Create Fusion+ Orders**: Create limit orders with optional Dutch auction
- **Order Management**: View and cancel your active orders
- **Resolver Dashboard**: Interface for resolver operators (coming soon)

## Setup

1. Install dependencies:
```bash
npm install
```

2. Configure contracts:
Edit `src/config/near.ts` to update contract addresses for your deployment.

3. Run development server:
```bash
npm run dev
```

4. Build for production:
```bash
npm run build
```

## Contract Integration

The UI interacts with the following NEAR contracts:
- `fusion-order`: Main limit order contract
- `fusion-resolver`: Cross-chain coordination contract
- `escrow-factory`: HTLC escrow deployment factory

## Usage

1. **Connect Wallet**: Click "Connect Wallet" to sign in with NEAR testnet
2. **Create Order**: 
   - Select tokens to swap
   - Enter amounts
   - Enable Dutch auction for dynamic pricing
   - Submit order
3. **Manage Orders**: View your active orders and cancel if needed
4. **Resolver Operations**: Coming soon - interface for cross-chain execution

## Development

The frontend uses:
- React + TypeScript
- Vite for bundling
- Tailwind CSS for styling
- near-api-js for blockchain interaction

## Testing

To test the full flow:
1. Deploy contracts to NEAR testnet
2. Update contract addresses in config
3. Connect testnet wallet with test tokens
4. Create and execute orders