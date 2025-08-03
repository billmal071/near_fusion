import React, { createContext, useContext, useEffect, useState } from 'react';
import { connect, Contract, keyStores, WalletConnection } from 'near-api-js';
import { CONTRACT_CONFIG } from '../config/near';

interface NearContextType {
  wallet: WalletConnection | null;
  account: any;
  fusionOrderContract: Contract | null;
  fusionResolverContract: Contract | null;
  escrowFactoryContract: Contract | null;
  signIn: () => void;
  signOut: () => void;
  isSignedIn: boolean;
}

const NearContext = createContext<NearContextType | undefined>(undefined);

export const useNear = () => {
  const context = useContext(NearContext);
  if (!context) {
    throw new Error('useNear must be used within NearProvider');
  }
  return context;
};

export const NearProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [wallet, setWallet] = useState<WalletConnection | null>(null);
  const [fusionOrderContract, setFusionOrderContract] = useState<Contract | null>(null);
  const [fusionResolverContract, setFusionResolverContract] = useState<Contract | null>(null);
  const [escrowFactoryContract, setEscrowFactoryContract] = useState<Contract | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const initNear = async () => {
      try {
        const keyStore = new keyStores.BrowserLocalStorageKeyStore();
      
      const nearConfig = {
        networkId: CONTRACT_CONFIG.networkId,
        keyStore,
        nodeUrl: CONTRACT_CONFIG.nodeUrl,
        walletUrl: CONTRACT_CONFIG.walletUrl,
        helperUrl: CONTRACT_CONFIG.helperUrl,
        headers: {}
      };

      const near = await connect(nearConfig);
      const walletConnection = new WalletConnection(near, 'near-fusion-app');
      setWallet(walletConnection);

      if (walletConnection.isSignedIn()) {
        const account = walletConnection.account();
        
        // Initialize contracts
        const fusionOrder = new Contract(account, CONTRACT_CONFIG.contracts.fusionOrder, {
          viewMethods: ['get_order', 'get_remaining_amounts'],
          changeMethods: ['create_order', 'fill_order', 'cancel_order', 'add_resolver', 'remove_resolver'],
          useLocalViewExecution: false
        });
        setFusionOrderContract(fusionOrder);

        const fusionResolver = new Contract(account, CONTRACT_CONFIG.contracts.fusionResolver, {
          viewMethods: [],
          changeMethods: ['initiate_cross_chain_swap', 'deploy_src_escrow', 'deploy_dst_escrow', 'execute_cross_chain_fill', 'add_operator', 'remove_operator'],
          useLocalViewExecution: false
        });
        setFusionResolverContract(fusionResolver);

        const escrowFactory = new Contract(account, CONTRACT_CONFIG.contracts.escrowFactory, {
          viewMethods: ['get_deployment'],
          changeMethods: ['create_src_escrow', 'create_dst_escrow'],
          useLocalViewExecution: false
        });
        setEscrowFactoryContract(escrowFactory);
      }
      } catch (error) {
        console.error('Failed to initialize NEAR:', error);
      } finally {
        setLoading(false);
      }
    };

    initNear();
  }, []);

  const signIn = () => {
    wallet?.requestSignIn({
      contractId: CONTRACT_CONFIG.contracts.fusionOrder,
      keyType: 'ed25519' as any
    });
  };

  const signOut = () => {
    wallet?.signOut();
    window.location.replace(window.location.origin);
  };

  const value = {
    wallet,
    account: wallet?.account(),
    fusionOrderContract,
    fusionResolverContract,
    escrowFactoryContract,
    signIn,
    signOut,
    isSignedIn: wallet?.isSignedIn() || false
  };

  if (loading) {
    return <div className="min-h-screen flex items-center justify-center">Loading NEAR...</div>;
  }

  return <NearContext.Provider value={value}>{children}</NearContext.Provider>;
};