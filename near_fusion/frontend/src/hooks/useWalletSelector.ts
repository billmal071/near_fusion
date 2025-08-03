import { useEffect, useState } from 'react';
import { CONTRACT_CONFIG } from '../config/near';

// Type imports
import type { WalletSelector } from '@near-wallet-selector/core';
import type { WalletSelectorModal } from '@near-wallet-selector/modal-ui';

export const useWalletSelector = () => {
  const [selector, setSelector] = useState<WalletSelector | null>(null);
  const [modal, setModal] = useState<WalletSelectorModal | null>(null);
  const [accountId, setAccountId] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const initSelector = async () => {
      try {
        // Dynamic imports to avoid ESM issues
        const { setupWalletSelector } = await import('@near-wallet-selector/core');
        const { setupModal } = await import('@near-wallet-selector/modal-ui');
        const { setupMyNearWallet } = await import('@near-wallet-selector/my-near-wallet');
        const { setupMeteorWallet } = await import('@near-wallet-selector/meteor-wallet');

        const walletSelector = await setupWalletSelector({
          network: CONTRACT_CONFIG.networkId as any,
          modules: [
            setupMyNearWallet(),
            setupMeteorWallet(),
          ],
        });

        const accounts = walletSelector.store.getState().accounts;
        const activeAccount = accounts.find((account: any) => account.active)?.accountId || null;
        
        setAccountId(activeAccount);
        setSelector(walletSelector);

        // Subscribe to account changes
        walletSelector.store.observable.subscribe((state: any) => {
          const activeAccount = state.accounts.find((account: any) => account.active)?.accountId || null;
          setAccountId(activeAccount);
        });

        // Setup modal
        const walletModal = setupModal(walletSelector, {
          contractId: CONTRACT_CONFIG.contracts.fusionOrder,
        });
        setModal(walletModal);
        
        setLoading(false);
      } catch (error) {
        console.error('Failed to initialize wallet selector:', error);
        setLoading(false);
      }
    };

    initSelector();
  }, []);

  const connectWallet = () => {
    modal?.show();
  };

  const disconnectWallet = async () => {
    if (!selector) return;
    
    const wallet = await selector.wallet();
    await wallet.signOut();
    setAccountId(null);
  };

  const viewMethod = async (contractId: string, methodName: string, args = {}) => {
    if (!selector) throw new Error('Wallet selector not initialized');
    
    const wallet = await selector.wallet();
    const accounts = await wallet.getAccounts();
    if (!accounts.length) throw new Error('No account available');
    
    // For view methods, we can use the RPC directly
    const { nodeUrl } = CONTRACT_CONFIG;
    const response = await fetch(nodeUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        jsonrpc: '2.0',
        id: 'dontcare',
        method: 'query',
        params: {
          request_type: 'call_function',
          finality: 'optimistic',
          account_id: contractId,
          method_name: methodName,
          args_base64: Buffer.from(JSON.stringify(args)).toString('base64')
        }
      })
    });
    
    return response.json();
  };

  const callMethod = async (
    contractId: string,
    methodName: string,
    args = {},
    gas = '30000000000000',
    deposit = '0'
  ) => {
    if (!selector || !accountId) throw new Error('Wallet not connected');
    
    const wallet = await selector.wallet();
    
    return wallet.signAndSendTransaction({
      signerId: accountId,
      receiverId: contractId,
      actions: [
        {
          type: 'FunctionCall',
          params: {
            methodName,
            args,
            gas,
            deposit
          }
        }
      ]
    });
  };

  return {
    selector,
    modal,
    accountId,
    loading,
    connectWallet,
    disconnectWallet,
    viewMethod,
    callMethod
  };
};