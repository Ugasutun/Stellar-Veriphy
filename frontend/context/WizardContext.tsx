"use client";

import { createContext, useContext, useState, ReactNode } from "react";

export type VerificationMode = "standard" | "advanced";

interface WizardContextValue {
  mode: VerificationMode | null;
  setMode: (mode: VerificationMode) => void;
  file: File | null;
  setFile: (file: File | null) => void;
  contentHash: string;
  setContentHash: (hash: string) => void;
  advancedContentHash: string;
  setAdvancedContentHash: (hash: string) => void;
  advancedManifestHash: string;
  setAdvancedManifestHash: (hash: string) => void;
  manifest: object | null;
  setManifest: (manifest: object | null) => void;
  manifestHash: string;
  setManifestHash: (hash: string) => void;
  hashProgress: number;
  setHashProgress: (progress: number) => void;
}

const WizardContext = createContext<WizardContextValue | undefined>(undefined);

export function WizardProvider({ children }: { children: ReactNode }) {
  const [mode, setMode] = useState<VerificationMode | null>(null);
  const [file, setFile] = useState<File | null>(null);
  const [contentHash, setContentHash] = useState("");
  const [advancedContentHash, setAdvancedContentHash] = useState("");
  const [advancedManifestHash, setAdvancedManifestHash] = useState("");
  const [manifest, setManifest] = useState<object | null>(null);
  const [manifestHash, setManifestHash] = useState("");
  const [hashProgress, setHashProgress] = useState(0);

  return (
    <WizardContext.Provider
      value={{
        mode,
        setMode,
        file,
        setFile,
        contentHash,
        setContentHash,
        advancedContentHash,
        setAdvancedContentHash,
        advancedManifestHash,
        setAdvancedManifestHash,
        manifest,
        setManifest,
        manifestHash,
        setManifestHash,
        hashProgress,
        setHashProgress,
      }}
    >
      {children}
    </WizardContext.Provider>
  );
}

export function useWizard() {
  const context = useContext(WizardContext);
  if (!context) {
    throw new Error("useWizard must be used within WizardProvider");
  }
  return context;
}
