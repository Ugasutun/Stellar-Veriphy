"use client";

import { useEffect, useState } from "react";
import { useWallet } from "@/context/WalletContext";

export function WalletModal() {
  const { publicKey, connected, error, connect, disconnect } = useWallet();
  const [isFreighterInstalled, setIsFreighterInstalled] = useState(false);

  useEffect(() => {
    // Check if Freighter is installed
    const checkFreighter = async () => {
      try {
        const result = await (window as any).freighter?.isAllowed?.();
        setIsFreighterInstalled(!!result || !!(window as any).freighter);
      } catch {
        setIsFreighterInstalled(!!(window as any).freighter);
      }
    };

    checkFreighter();
  }, []);

  const truncateKey = (key: string) => {
    return `${key.slice(0, 6)}...${key.slice(-6)}`;
  };

  return (
    <div className="flex flex-col gap-4 p-4 bg-slate-900 rounded-lg border border-slate-800">
      {error && (
        <div className="p-3 bg-red-900/20 border border-red-800 rounded text-red-400 text-sm">
          {error}
        </div>
      )}

      {!isFreighterInstalled ? (
        <div className="text-center">
          <p className="text-slate-300 mb-3">Freighter wallet not detected</p>
          <a
            href="https://www.freighter.app/"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-block px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors"
          >
            Install Freighter
          </a>
        </div>
      ) : connected && publicKey ? (
        <div className="space-y-3">
          <div className="p-3 bg-slate-800 rounded">
            <p className="text-xs text-slate-400 mb-1">Connected Wallet</p>
            <p className="text-white font-mono">{truncateKey(publicKey)}</p>
          </div>
          <button
            onClick={disconnect}
            className="w-full px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded transition-colors"
          >
            Disconnect
          </button>
        </div>
      ) : (
        <button
          onClick={connect}
          className="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors"
        >
          Connect Wallet
        </button>
      )}
    </div>
  );
}
