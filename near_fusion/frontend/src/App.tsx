import { useState } from 'react'
import { TOKENS } from './config/near'
import './App.css'

function App() {
  const [activeTab, setActiveTab] = useState<'create' | 'orders' | 'resolver'>('create');
  const [isConnected, setIsConnected] = useState(false);
  const [accountId, setAccountId] = useState<string>('');

  // Mock wallet connection for now
  const connectWallet = () => {
    // In a real implementation, this would use @near-wallet-selector
    const mockAccount = `user${Math.floor(Math.random() * 1000)}.testnet`;
    setAccountId(mockAccount);
    setIsConnected(true);
  };

  const disconnectWallet = () => {
    setAccountId('');
    setIsConnected(false);
  };

  return (
    <div className="min-h-screen bg-gray-100">
      <header className="bg-white shadow">
        <div className="container mx-auto px-4 py-6">
          <div className="flex justify-between items-center">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">NEAR Fusion+</h1>
              <p className="text-sm text-gray-600">1inch Cross-Chain Swaps on NEAR</p>
            </div>
            <div>
              {isConnected ? (
                <div className="flex items-center gap-4">
                  <span className="text-sm text-gray-700">
                    Connected: <span className="font-medium">{accountId}</span>
                  </span>
                  <button
                    onClick={disconnectWallet}
                    className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition"
                  >
                    Disconnect
                  </button>
                </div>
              ) : (
                <button
                  onClick={connectWallet}
                  className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
                >
                  Connect Wallet (Mock)
                </button>
              )}
            </div>
          </div>
        </div>
      </header>
      
      <div className="container mx-auto px-4 py-8">
        <div className="flex justify-center mb-8">
          <div className="inline-flex rounded-lg shadow">
            <button
              onClick={() => setActiveTab('create')}
              className={`px-6 py-3 rounded-l-lg font-medium ${
                activeTab === 'create' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-white text-gray-700 hover:bg-gray-50'
              }`}
            >
              Create Order
            </button>
            <button
              onClick={() => setActiveTab('orders')}
              className={`px-6 py-3 font-medium ${
                activeTab === 'orders' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-white text-gray-700 hover:bg-gray-50'
              }`}
            >
              My Orders
            </button>
            <button
              onClick={() => setActiveTab('resolver')}
              className={`px-6 py-3 rounded-r-lg font-medium ${
                activeTab === 'resolver' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-white text-gray-700 hover:bg-gray-50'
              }`}
            >
              Resolver
            </button>
          </div>
        </div>

        <div className="max-w-6xl mx-auto">
          {activeTab === 'create' && (
            <div className="bg-white rounded-lg shadow p-6">
              <h2 className="text-2xl font-bold mb-4">Create Order</h2>
              {isConnected ? (
                <CreateOrderForm />
              ) : (
                <p className="text-gray-600">Please connect your wallet to create orders</p>
              )}
            </div>
          )}
          {activeTab === 'orders' && (
            <div className="bg-white rounded-lg shadow p-6">
              <h2 className="text-2xl font-bold mb-4">My Orders</h2>
              {isConnected ? (
                <OrderList accountId={accountId} />
              ) : (
                <p className="text-gray-600">Please connect your wallet to view orders</p>
              )}
            </div>
          )}
          {activeTab === 'resolver' && (
            <div className="bg-white rounded-lg shadow p-6">
              <h2 className="text-2xl font-bold mb-4">Resolver Dashboard</h2>
              <p className="text-gray-600">Coming soon...</p>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

// Create Order Form component
function CreateOrderForm() {
  const [makerAsset, setMakerAsset] = useState(TOKENS[0].id);
  const [takerAsset, setTakerAsset] = useState(TOKENS[1].id);
  const [makingAmount, setMakingAmount] = useState('');
  const [takingAmount, setTakingAmount] = useState('');
  const [enableAuction, setEnableAuction] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    console.log('Creating order:', { 
      makerAsset, 
      takerAsset, 
      makingAmount, 
      takingAmount,
      enableAuction 
    });
    alert('Order creation will be implemented with deployed contracts');
  };

  const makerToken = TOKENS.find(t => t.id === makerAsset);
  const takerToken = TOKENS.find(t => t.id === takerAsset);

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          You're Selling
        </label>
        <select
          value={makerAsset}
          onChange={(e) => setMakerAsset(e.target.value)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg mb-2"
        >
          {TOKENS.map(token => (
            <option key={token.id} value={token.id}>
              {token.symbol} - {token.name}
            </option>
          ))}
        </select>
        <input
          type="number"
          value={makingAmount}
          onChange={(e) => setMakingAmount(e.target.value)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg"
          placeholder="Amount"
        />
        {makerToken && makingAmount && (
          <p className="text-sm text-gray-600 mt-1">
            {(parseFloat(makingAmount) / Math.pow(10, makerToken.decimals)).toFixed(6)} {makerToken.symbol}
          </p>
        )}
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          You're Buying
        </label>
        <select
          value={takerAsset}
          onChange={(e) => setTakerAsset(e.target.value)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg mb-2"
        >
          {TOKENS.map(token => (
            <option key={token.id} value={token.id}>
              {token.symbol} - {token.name}
            </option>
          ))}
        </select>
        <input
          type="number"
          value={takingAmount}
          onChange={(e) => setTakingAmount(e.target.value)}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg"
          placeholder="Amount"
        />
        {takerToken && takingAmount && (
          <p className="text-sm text-gray-600 mt-1">
            {(parseFloat(takingAmount) / Math.pow(10, takerToken.decimals)).toFixed(6)} {takerToken.symbol}
          </p>
        )}
      </div>

      <div className="border-t pt-4">
        <label className="flex items-center gap-3">
          <input
            type="checkbox"
            checked={enableAuction}
            onChange={(e) => setEnableAuction(e.target.checked)}
            className="w-4 h-4 text-blue-600 rounded"
          />
          <span className="font-medium">Enable Dutch Auction</span>
        </label>
      </div>

      <button
        type="submit"
        className="w-full py-3 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700"
      >
        Create Order
      </button>
    </form>
  );
}

// Order List component
function OrderList({ accountId }: { accountId: string }) {
  const mockOrders = [
    {
      id: '1',
      makerAsset: TOKENS[0],
      takerAsset: TOKENS[1],
      makingAmount: '1000000000000000000000000',
      takingAmount: '1000000',
      status: 'active',
      filledPercent: 0
    },
    {
      id: '2',
      makerAsset: TOKENS[1],
      takerAsset: TOKENS[0],
      makingAmount: '5000000',
      takingAmount: '5000000000000000000000000',
      status: 'active',
      filledPercent: 30
    }
  ];

  return (
    <div className="space-y-4">
      <p className="text-sm text-gray-600 mb-4">Showing mock orders for demonstration</p>
      {mockOrders.map(order => (
        <div key={order.id} className="border rounded-lg p-4">
          <div className="flex justify-between items-start mb-2">
            <div>
              <p className="font-medium">
                {(parseFloat(order.makingAmount) / Math.pow(10, order.makerAsset.decimals)).toFixed(4)} {order.makerAsset.symbol}
                {' → '}
                {(parseFloat(order.takingAmount) / Math.pow(10, order.takerAsset.decimals)).toFixed(4)} {order.takerAsset.symbol}
              </p>
            </div>
            <span className="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm">
              Active
            </span>
          </div>
          {order.filledPercent > 0 && (
            <div className="mt-2">
              <div className="flex justify-between text-sm text-gray-600 mb-1">
                <span>Filled</span>
                <span>{order.filledPercent}%</span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-2">
                <div 
                  className="bg-blue-600 h-2 rounded-full"
                  style={{ width: `${order.filledPercent}%` }}
                />
              </div>
            </div>
          )}
          <button className="mt-3 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 text-sm">
            Cancel Order
          </button>
        </div>
      ))}
    </div>
  );
}

export default App