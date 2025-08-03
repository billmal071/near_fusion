import { useState } from 'react';
import { useNear } from '../contexts/NearContext';
import { TOKENS } from '../config/near';
import type { FusionOrder, AuctionDetails } from '../types/contracts';

const CreateOrder = () => {
  const { fusionOrderContract, isSignedIn } = useNear();
  
  const [makerAsset, setMakerAsset] = useState(TOKENS[0].id);
  const [takerAsset, setTakerAsset] = useState(TOKENS[1].id);
  const [makingAmount, setMakingAmount] = useState('');
  const [takingAmount, setTakingAmount] = useState('');
  const [enableAuction, setEnableAuction] = useState(false);
  const [auctionDuration, setAuctionDuration] = useState('3600');
  const [initialRateBump, setInitialRateBump] = useState('200'); // 2%
  const [loading, setLoading] = useState(false);

  const handleCreateOrder = async () => {
    if (!isSignedIn || !fusionOrderContract) {
      alert('Please connect your wallet first');
      return;
    }

    if (!makingAmount || !takingAmount) {
      alert('Please enter amounts');
      return;
    }

    setLoading(true);
    try {
      const salt = new Uint8Array(32);
      crypto.getRandomValues(salt);
      
      const order: FusionOrder = {
        order_hash: Array(32).fill(0).join(''), // Will be computed by contract
        maker: fusionOrderContract.account?.accountId || '',
        resolver: undefined,
        maker_asset: makerAsset,
        taker_asset: takerAsset,
        making_amount: makingAmount,
        taking_amount: takingAmount,
        maker_traits: 0,
        salt: Array.from(salt).map(b => b.toString(16).padStart(2, '0')).join(''),
      };

      let auction: AuctionDetails | undefined;
      if (enableAuction) {
        auction = {
          start_time: Math.floor(Date.now() / 1000),
          duration: parseInt(auctionDuration),
          initial_rate_bump: parseInt(initialRateBump),
          points: [
            { delay: 900, coefficient: 750000 }, // 75% after 15 min
            { delay: 1800, coefficient: 500000 }, // 50% after 30 min
          ],
        };
      }

      // @ts-ignore
      await fusionOrderContract.create_order({
        order,
        auction,
      });

      alert('Order created successfully!');
      
      // Reset form
      setMakingAmount('');
      setTakingAmount('');
      setEnableAuction(false);
    } catch (error) {
      console.error('Error creating order:', error);
      alert('Failed to create order');
    } finally {
      setLoading(false);
    }
  };

  const makerToken = TOKENS.find(t => t.id === makerAsset);
  const takerToken = TOKENS.find(t => t.id === takerAsset);

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-2xl font-bold mb-6">Create Fusion+ Order</h2>
      
      <div className="space-y-6">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            You're Selling
          </label>
          <div className="flex gap-4">
            <select
              value={makerAsset}
              onChange={(e) => setMakerAsset(e.target.value)}
              className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              {TOKENS.map(token => (
                <option key={token.id} value={token.id}>
                  {token.symbol} - {token.name}
                </option>
              ))}
            </select>
            <input
              type="number"
              placeholder="Amount"
              value={makingAmount}
              onChange={(e) => setMakingAmount(e.target.value)}
              className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
          {makerToken && makingAmount && (
            <p className="text-sm text-gray-600 mt-1">
              {(parseFloat(makingAmount) / Math.pow(10, makerToken.decimals)).toFixed(6)} {makerToken.symbol}
            </p>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            You're Buying
          </label>
          <div className="flex gap-4">
            <select
              value={takerAsset}
              onChange={(e) => setTakerAsset(e.target.value)}
              className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              {TOKENS.map(token => (
                <option key={token.id} value={token.id}>
                  {token.symbol} - {token.name}
                </option>
              ))}
            </select>
            <input
              type="number"
              placeholder="Amount"
              value={takingAmount}
              onChange={(e) => setTakingAmount(e.target.value)}
              className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
          {takerToken && takingAmount && (
            <p className="text-sm text-gray-600 mt-1">
              {(parseFloat(takingAmount) / Math.pow(10, takerToken.decimals)).toFixed(6)} {takerToken.symbol}
            </p>
          )}
        </div>

        <div className="border-t pt-6">
          <label className="flex items-center gap-3">
            <input
              type="checkbox"
              checked={enableAuction}
              onChange={(e) => setEnableAuction(e.target.checked)}
              className="w-4 h-4 text-blue-600 rounded focus:ring-blue-500"
            />
            <span className="font-medium">Enable Dutch Auction</span>
          </label>
          
          {enableAuction && (
            <div className="mt-4 space-y-4 pl-7">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Duration (seconds)
                </label>
                <input
                  type="number"
                  value={auctionDuration}
                  onChange={(e) => setAuctionDuration(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Initial Rate Bump (basis points)
                </label>
                <input
                  type="number"
                  value={initialRateBump}
                  onChange={(e) => setInitialRateBump(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
                <p className="text-sm text-gray-600 mt-1">
                  {(parseInt(initialRateBump) / 100).toFixed(2)}% initial bonus
                </p>
              </div>
            </div>
          )}
        </div>

        <button
          onClick={handleCreateOrder}
          disabled={loading || !isSignedIn}
          className="w-full py-3 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 transition disabled:bg-gray-400"
        >
          {loading ? 'Creating Order...' : 'Create Order'}
        </button>
      </div>
    </div>
  );
};

export default CreateOrder;