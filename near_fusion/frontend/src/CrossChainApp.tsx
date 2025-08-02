import { useState, useEffect } from 'react'
import { ConnectButton } from '@rainbow-me/rainbowkit'
import { useAccount as useEthAccount, useBalance } from 'wagmi'
import { useWalletSelector } from './hooks/useWalletSelector'
import { TOKENS, mockContractCalls } from './config/near'
import './App.css'
import '@near-wallet-selector/modal-ui/styles.css'

// Ethereum tokens
const ETH_TOKENS = [
  { id: '0x0', symbol: 'ETH', name: 'Ethereum', decimals: 18 },
  { id: '0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48', symbol: 'USDC', name: 'USD Coin', decimals: 6 },
  { id: '0x6B175474E89094C44Da98b954EedeAC495271d0F', symbol: 'DAI', name: 'Dai', decimals: 18 },
];

function CrossChainApp() {
  const [activeTab, setActiveTab] = useState<'create' | 'orders' | 'bridge'>('bridge');
  const [sourceChain, setSourceChain] = useState<'ethereum' | 'near'>('ethereum');
  const [targetChain, setTargetChain] = useState<'ethereum' | 'near'>('near');
  
  // Ethereum wallet
  const { address: ethAddress, isConnected: ethConnected } = useEthAccount();
  
  // NEAR wallet
  const { accountId: nearAccountId, loading: nearLoading, connectWallet: connectNear, disconnectWallet: disconnectNear } = useWalletSelector();

  return (
    <div className="min-h-screen bg-gray-100">
      <header className="bg-white shadow">
        <div className="container mx-auto px-4 py-6">
          <div className="flex justify-between items-center">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">1inch Fusion+ Bridge</h1>
              <p className="text-sm text-gray-600">Cross-chain swaps between Ethereum and NEAR</p>
            </div>
            <div className="flex gap-4">
              {/* Ethereum Wallet */}
              <div>
                <p className="text-xs text-gray-500 mb-1">Ethereum</p>
                <ConnectButton />
              </div>
              
              {/* NEAR Wallet */}
              <div>
                <p className="text-xs text-gray-500 mb-1">NEAR</p>
                {nearLoading ? (
                  <span className="text-gray-500">Loading...</span>
                ) : nearAccountId ? (
                  <div className="flex items-center gap-2">
                    <span className="text-sm">{nearAccountId}</span>
                    <button
                      onClick={disconnectNear}
                      className="px-3 py-1 bg-red-600 text-white rounded text-sm"
                    >
                      Disconnect
                    </button>
                  </div>
                ) : (
                  <button
                    onClick={connectNear}
                    className="px-4 py-2 bg-blue-600 text-white rounded"
                  >
                    Connect NEAR
                  </button>
                )}
              </div>
            </div>
          </div>
        </div>
      </header>
      
      <div className="container mx-auto px-4 py-8">
        <div className="flex justify-center mb-8">
          <div className="inline-flex rounded-lg shadow">
            <button
              onClick={() => setActiveTab('bridge')}
              className={`px-6 py-3 rounded-l-lg font-medium ${
                activeTab === 'bridge' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-white text-gray-700 hover:bg-gray-50'
              }`}
            >
              Bridge Swap
            </button>
            <button
              onClick={() => setActiveTab('create')}
              className={`px-6 py-3 font-medium ${
                activeTab === 'create' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-white text-gray-700 hover:bg-gray-50'
              }`}
            >
              Create Order
            </button>
            <button
              onClick={() => setActiveTab('orders')}
              className={`px-6 py-3 rounded-r-lg font-medium ${
                activeTab === 'orders' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-white text-gray-700 hover:bg-gray-50'
              }`}
            >
              My Orders
            </button>
          </div>
        </div>

        <div className="max-w-6xl mx-auto">
          {activeTab === 'bridge' && (
            <BridgeInterface 
              ethConnected={ethConnected}
              nearConnected={!!nearAccountId}
              sourceChain={sourceChain}
              targetChain={targetChain}
              setSourceChain={setSourceChain}
              setTargetChain={setTargetChain}
            />
          )}
          {activeTab === 'create' && (
            <div className="bg-white rounded-lg shadow p-6">
              <h2 className="text-2xl font-bold mb-4">Create Cross-Chain Order</h2>
              {ethConnected && nearAccountId ? (
                <CreateCrossChainOrder 
                  sourceChain={sourceChain}
                  targetChain={targetChain}
                />
              ) : (
                <p className="text-gray-600">Please connect both Ethereum and NEAR wallets</p>
              )}
            </div>
          )}
          {activeTab === 'orders' && (
            <div className="bg-white rounded-lg shadow p-6">
              <h2 className="text-2xl font-bold mb-4">My Cross-Chain Orders</h2>
              <OrdersList ethAddress={ethAddress} nearAccountId={nearAccountId} />
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

// Bridge Interface Component
function BridgeInterface({ 
  ethConnected, 
  nearConnected, 
  sourceChain, 
  targetChain,
  setSourceChain,
  setTargetChain 
}: any) {
  const [sourceToken, setSourceToken] = useState('');
  const [targetToken, setTargetToken] = useState('');
  const [amount, setAmount] = useState('');

  const handleSwapChains = () => {
    const temp = sourceChain;
    setSourceChain(targetChain);
    setTargetChain(temp);
  };

  const sourceTokens = sourceChain === 'ethereum' ? ETH_TOKENS : TOKENS;
  const targetTokens = targetChain === 'ethereum' ? ETH_TOKENS : TOKENS;

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-2xl font-bold mb-6">Cross-Chain Bridge</h2>
      
      <div className="space-y-4">
        {/* Source */}
        <div className="border rounded-lg p-4">
          <div className="flex justify-between items-center mb-2">
            <label className="text-sm font-medium text-gray-700">From</label>
            <span className="text-sm text-gray-500 capitalize">{sourceChain}</span>
          </div>
          <select
            value={sourceToken}
            onChange={(e) => setSourceToken(e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg mb-2"
          >
            <option value="">Select token</option>
            {sourceTokens.map(token => (
              <option key={token.id} value={token.id}>
                {token.symbol} - {token.name}
              </option>
            ))}
          </select>
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg"
            placeholder="Amount"
          />
        </div>

        {/* Swap Button */}
        <div className="flex justify-center">
          <button
            onClick={handleSwapChains}
            className="p-2 rounded-full bg-gray-100 hover:bg-gray-200 transition"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" />
            </svg>
          </button>
        </div>

        {/* Target */}
        <div className="border rounded-lg p-4">
          <div className="flex justify-between items-center mb-2">
            <label className="text-sm font-medium text-gray-700">To</label>
            <span className="text-sm text-gray-500 capitalize">{targetChain}</span>
          </div>
          <select
            value={targetToken}
            onChange={(e) => setTargetToken(e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg"
          >
            <option value="">Select token</option>
            {targetTokens.map(token => (
              <option key={token.id} value={token.id}>
                {token.symbol} - {token.name}
              </option>
            ))}
          </select>
        </div>

        {/* Action Button */}
        <button
          disabled={!ethConnected || !nearConnected || !sourceToken || !targetToken || !amount}
          className="w-full py-3 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 disabled:bg-gray-400"
        >
          {!ethConnected || !nearConnected 
            ? 'Connect Both Wallets' 
            : 'Create Bridge Order'}
        </button>

        {/* Info */}
        <div className="mt-4 p-4 bg-blue-50 rounded-lg">
          <h3 className="font-medium text-blue-900 mb-2">How it works:</h3>
          <ol className="text-sm text-blue-800 space-y-1 list-decimal list-inside">
            <li>Create a cross-chain order specifying source and target assets</li>
            <li>Resolvers match your order with opposing orders</li>
            <li>HTLC contracts ensure atomic swap execution</li>
            <li>Receive your target chain assets seamlessly</li>
          </ol>
        </div>
      </div>
    </div>
  );
}

// Create Cross-Chain Order Component
function CreateCrossChainOrder({ sourceChain, targetChain }: any) {
  return (
    <div>
      <p>Create orders between {sourceChain} and {targetChain}</p>
      {/* Implementation similar to existing CreateOrderForm but for cross-chain */}
    </div>
  );
}

// Orders List Component
function OrdersList({ ethAddress, nearAccountId }: any) {
  return (
    <div>
      {ethAddress && (
        <div className="mb-4">
          <h3 className="font-medium mb-2">Ethereum Orders</h3>
          <p className="text-sm text-gray-600">Address: {ethAddress}</p>
        </div>
      )}
      {nearAccountId && (
        <div>
          <h3 className="font-medium mb-2">NEAR Orders</h3>
          <p className="text-sm text-gray-600">Account: {nearAccountId}</p>
        </div>
      )}
    </div>
  );
}

export default CrossChainApp;