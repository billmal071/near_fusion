import { useState, useEffect } from 'react';
import { useNear } from '../contexts/NearContext';
import { OrderState } from '../types/contracts';
import { TOKENS } from '../config/near';

const OrderList = () => {
  const { fusionOrderContract, account, isSignedIn } = useNear();
  const [orders, setOrders] = useState<{ hash: string; state: OrderState }[]>([]);
  const [loading, setLoading] = useState(false);

  const loadOrders = async () => {
    if (!fusionOrderContract || !account) return;
    
    setLoading(true);
    try {
      // In a real implementation, we'd need to query orders by maker
      // For now, this is a placeholder
      console.log('Loading orders for:', account.accountId);
      // const orderList = await fusionOrderContract.get_orders_by_maker({ maker: account.accountId });
      // setOrders(orderList);
    } catch (error) {
      console.error('Error loading orders:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (isSignedIn) {
      loadOrders();
    }
  }, [isSignedIn, account]);

  const handleCancelOrder = async (orderHash: string) => {
    if (!fusionOrderContract) return;
    
    try {
      // @ts-ignore
      await fusionOrderContract.cancel_order({ order_hash: orderHash });
      alert('Order cancelled successfully');
      loadOrders();
    } catch (error) {
      console.error('Error cancelling order:', error);
      alert('Failed to cancel order');
    }
  };

  const getTokenSymbol = (tokenId: string) => {
    const token = TOKENS.find(t => t.id === tokenId);
    return token?.symbol || tokenId;
  };

  const formatAmount = (amount: string, tokenId: string) => {
    const token = TOKENS.find(t => t.id === tokenId);
    if (!token) return amount;
    return (parseFloat(amount) / Math.pow(10, token.decimals)).toFixed(6);
  };

  if (!isSignedIn) {
    return (
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-2xl font-bold mb-4">My Orders</h2>
        <p className="text-gray-600">Please connect your wallet to view orders</p>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-2xl font-bold mb-6">My Orders</h2>
      
      {loading ? (
        <p className="text-gray-600">Loading orders...</p>
      ) : orders.length === 0 ? (
        <p className="text-gray-600">No orders found</p>
      ) : (
        <div className="space-y-4">
          {orders.map(({ hash, state }) => (
            <div key={hash} className="border rounded-lg p-4">
              <div className="flex justify-between items-start mb-4">
                <div>
                  <p className="text-sm text-gray-600">Order Hash</p>
                  <p className="font-mono text-xs">{hash}</p>
                </div>
                <span className={`px-3 py-1 rounded-full text-sm ${
                  state.is_cancelled 
                    ? 'bg-red-100 text-red-800' 
                    : 'bg-green-100 text-green-800'
                }`}>
                  {state.is_cancelled ? 'Cancelled' : 'Active'}
                </span>
              </div>
              
              <div className="grid grid-cols-2 gap-4 mb-4">
                <div>
                  <p className="text-sm text-gray-600">Selling</p>
                  <p className="font-medium">
                    {formatAmount(state.order.making_amount, state.order.maker_asset)} {getTokenSymbol(state.order.maker_asset)}
                  </p>
                  {state.filled_making_amount !== '0' && (
                    <p className="text-xs text-gray-500">
                      Filled: {formatAmount(state.filled_making_amount, state.order.maker_asset)}
                    </p>
                  )}
                </div>
                <div>
                  <p className="text-sm text-gray-600">Buying</p>
                  <p className="font-medium">
                    {formatAmount(state.order.taking_amount, state.order.taker_asset)} {getTokenSymbol(state.order.taker_asset)}
                  </p>
                  {state.filled_taking_amount !== '0' && (
                    <p className="text-xs text-gray-500">
                      Filled: {formatAmount(state.filled_taking_amount, state.order.taker_asset)}
                    </p>
                  )}
                </div>
              </div>
              
              {state.auction && (
                <div className="mb-4 p-3 bg-blue-50 rounded">
                  <p className="text-sm font-medium text-blue-900">Dutch Auction Active</p>
                  <p className="text-xs text-blue-700">
                    Initial bonus: {(state.auction.initial_rate_bump / 100).toFixed(2)}%
                  </p>
                </div>
              )}
              
              {!state.is_cancelled && (
                <button
                  onClick={() => handleCancelOrder(hash)}
                  className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition"
                >
                  Cancel Order
                </button>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default OrderList;