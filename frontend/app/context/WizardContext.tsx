"use client";

import { createContext, useContext, useState, useCallback } from "react";
import { ContentManifest } from "@stellarveriphy/shared/types";

interface WizardContextValue {
  manifest: Partial<ContentManifest>;
  setManifest: (manifest: Partial<ContentManifest>) => void;
  encryptionEnabled: boolean;
  setEncryptionEnabled: (enabled: boolean) => void;
  spvResult: {
    certificateId?: string;
    manifestHash?: string;
    contentHash?: string;
    attestationHash?: string;
    transactionHash?: string;
    success?: boolean;
  };
  setSPVResult: (result: any) => void;
  reset: () => void;
}

const WizardContext = createContext<WizardContextValue>({
  manifest: {},
  setManifest: () => {},
  encryptionEnabled: false,
  setEncryptionEnabled: () => {},
  spvResult: {},
  setSPVResult: () => {},
  reset: () => {},
});

export function WizardProvider({ children }: { children: React.ReactNode }) {
  const [manifest, setManifest] = useState<Partial<ContentManifest>>({});
  const [encryptionEnabled, setEncryptionEnabled] = useState(false);
  const [spvResult, setSPVResult] = useState({});

  const reset = useCallback(() => {
    setManifest({});
    setEncryptionEnabled(false);
    setSPVResult({});
  }, []);

  return (
    <WizardContext.Provider
      value={{
        manifest,
        setManifest,
        encryptionEnabled,
        setEncryptionEnabled,
        spvResult,
        setSPVResult,
        reset,
      }}
    >
      {children}
    </WizardContext.Provider>
  );
}

export const useWizard = () => useContext(WizardContext);
