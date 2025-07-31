import { useEffect, useState } from 'react';
import { setupWalletSelector, WalletSelector, AccountState } from '@near-wallet-selector/core';
import { setupModal, WalletSelectorModal } from '@near-wallet-selector/modal-ui';
import { setupMyNearWallet } from '@near-wallet-selector/my-near-wallet';
import { setupMeteorWallet } from '@near-wallet-selector/meteor-wallet';
import { CONTRACT_CONFIG } from '../config/near';
import type { Account } from 'near-api-js';

export const useWalletSelector = () => {
  const [selector, setSelector] = useState<WalletSelector | null>(null);
  const [modal, setModal] = useState<WalletSelectorModal | null>(null);
  const [accounts, setAccounts] = useState<AccountState[]>([]);
  const [accountId, setAccountId] = useState<string | null>(null);
  const [account, setAccount] = useState<Account | null>(null);

  const init = async () => {
    const _selector = await setupWalletSelector({
      network: CONTRACT_CONFIG.networkId,
      modules: [
        setupMyNearWallet(),
        setupMeteorWallet(),
      ],
    });

    const _modal = setupModal(_selector, {
      contractId: CONTRACT_CONFIG.contracts.fusionOrder,
    });

    const state = _selector.store.getState();
    setAccounts(state.accounts);

    setSelector(_selector);
    setModal(_modal);
  };

  useEffect(() => {
    init();
  }, []);

  useEffect(() => {
    if (!selector) return;

    const subscription = selector.store.observable
      .subscribe((state) => {
        setAccounts(state.accounts);
        setAccountId(state.accounts.find((account) => account.active)?.accountId || null);
      });

    return () => subscription.unsubscribe();
  }, [selector]);

  useEffect(() => {
    const getAccount = async () => {
      if (!selector || !accountId) {
        setAccount(null);
        return;
      }

      const wallet = await selector.wallet();
      const walletAccounts = await wallet.getAccounts();
      const activeAccount = walletAccounts.find(acc => acc.accountId === accountId);
      
      if (activeAccount) {
        // For view methods, we can use the selector directly
        // For change methods, we'll use wallet.signAndSendTransaction
        setAccount(activeAccount as any);
      }
    };

    getAccount();
  }, [selector, accountId]);

  const showModal = () => {
    modal?.show();
  };

  const signOut = async () => {
    const wallet = await selector?.wallet();
    wallet?.signOut();
  };

  const callViewMethod = async (contractId: string, methodName: string, args: any = {}) => {
    if (!selector) throw new Error('Wallet not initialized');
    
    const { network } = selector.options;
    const provider = new (await import('near-api-js')).providers.JsonRpcProvider({ url: network.nodeUrl });

    const res = await provider.query({
      request_type: 'call_function',
      account_id: contractId,
      method_name: methodName,
      args_base64: Buffer.from(JSON.stringify(args)).toString('base64'),
      finality: 'optimistic',
    });

    // @ts-ignore
    return JSON.parse(Buffer.from(res.result).toString());
  };

  const callChangeMethod = async (
    contractId: string,
    methodName: string,
    args: any = {},
    gas: string = '30000000000000',
    deposit: string = '0'
  ) => {
    if (!selector || !accountId) throw new Error('Wallet not connected');

    const wallet = await selector.wallet();
    return wallet.signAndSendTransaction({
      receiverId: contractId,
      actions: [
        {
          type: 'FunctionCall',
          params: {
            methodName,
            args,
            gas,
            deposit,
          },
        },
      ],
    });
  };

  return {
    selector,
    modal,
    accounts,
    accountId,
    account,
    showModal,
    signOut,
    callViewMethod,
    callChangeMethod,
  };
};