"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import {
  getAddress,
  getNetworkDetails,
  isConnected,
  signTransaction,
} from "@stellar/freighter-api";

const STORAGE_KEY = "freighter_public_key";
const POLL_INTERVAL = 4000;

interface NetworkDetails {
  network: string;
  networkUrl: string;
  networkPassphrase: string;
  sorobanRpcUrl?: string;
}

interface WalletContextValue {
  publicKey: string | null;
  network: NetworkDetails | null;
  connected: boolean;
  error: string | null;
  connect: () => Promise<void>;
  disconnect: () => void;
  signTx: (xdr: string) => Promise<string>;
  clearError: () => void;
  refreshNetwork: () => Promise<void>;
}

const WalletContext = createContext<WalletContextValue>({
  publicKey: null,
  network: null,
  connected: false,
  error: null,
  connect: async () => {},
  disconnect: () => {},
  signTx: async () => "",
  clearError: () => {},
  refreshNetwork: async () => {},
});

export function WalletProvider({ children }: { children: React.ReactNode }) {
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [network, setNetwork] = useState<NetworkDetails | null>(null);
  const [error, setError] = useState<string | null>(null);
  const pollRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const refreshNetwork = useCallback(async () => {
    const res = await getNetworkDetails();
    if (res.error) {
      setError(res.error.message);
    } else {
      setNetwork({
        network: res.network,
        networkUrl: res.networkUrl,
        networkPassphrase: res.networkPassphrase,
        sorobanRpcUrl: res.sorobanRpcUrl,
      });
    }
  }, []);

  const startPolling = useCallback(() => {
    if (pollRef.current) return;
    pollRef.current = setInterval(refreshNetwork, POLL_INTERVAL);
  }, [refreshNetwork]);

  const stopPolling = useCallback(() => {
    if (pollRef.current) {
      clearInterval(pollRef.current);
      pollRef.current = null;
    }
  }, []);

  // Auto-reconnect from localStorage on mount
  useEffect(() => {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (!saved) return;
    isConnected().then(({ isConnected: ok }) => {
      if (ok) {
        setPublicKey(saved);
        refreshNetwork();
        startPolling();
      } else {
        localStorage.removeItem(STORAGE_KEY);
      }
    });
    return () => stopPolling();
  }, [refreshNetwork, startPolling, stopPolling]);

  const connect = useCallback(async () => {
    try {
      const { address, error: err } = await getAddress();
      if (err) throw new Error(err.message);
      localStorage.setItem(STORAGE_KEY, address);
      setPublicKey(address);
      setError(null);
      await refreshNetwork();
      startPolling();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to connect wallet");
    }
  }, [refreshNetwork, startPolling]);

  const disconnect = useCallback(() => {
    localStorage.removeItem(STORAGE_KEY);
    setPublicKey(null);
    setNetwork(null);
    setError(null);
    stopPolling();
  }, [stopPolling]);

  const signTx = useCallback(
    async (xdr: string): Promise<string> => {
      if (!publicKey) throw new Error("Wallet not connected");
      const { signedTxXdr, error: err } = await signTransaction(xdr, {
        networkPassphrase: network?.networkPassphrase,
        address: publicKey,
      });
      if (err) throw new Error(err.message);
      return signedTxXdr;
    },
    [publicKey, network]
  );

  const clearError = useCallback(() => setError(null), []);

  return (
    <WalletContext.Provider
      value={{
        publicKey,
        network,
        connected: !!publicKey,
        error,
        connect,
        disconnect,
        signTx,
        clearError,
        refreshNetwork,
      }}
    >
      {children}
    </WalletContext.Provider>
  );
}

export const useWallet = () => useContext(WalletContext);
