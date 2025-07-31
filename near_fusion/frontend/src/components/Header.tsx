import { useNear } from '../contexts/NearContext';

const Header = () => {
  let account, signIn, signOut, isSignedIn;
  
  try {
    const nearContext = useNear();
    account = nearContext.account;
    signIn = nearContext.signIn;
    signOut = nearContext.signOut;
    isSignedIn = nearContext.isSignedIn;
  } catch (error) {
    // Context not ready yet
    return null;
  }

  return (
    <header className="bg-white shadow">
      <div className="container mx-auto px-4 py-6">
        <div className="flex justify-between items-center">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">NEAR Fusion+</h1>
            <p className="text-sm text-gray-600">1inch Cross-Chain Swaps on NEAR</p>
          </div>
          
          <div>
            {isSignedIn ? (
              <div className="flex items-center gap-4">
                <span className="text-sm text-gray-700">
                  Connected: <span className="font-medium">{account?.accountId}</span>
                </span>
                <button
                  onClick={signOut}
                  className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition"
                >
                  Disconnect
                </button>
              </div>
            ) : (
              <button
                onClick={signIn}
                className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
              >
                Connect Wallet
              </button>
            )}
          </div>
        </div>
      </div>
    </header>
  );
};

export default Header;