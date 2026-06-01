"use client";

import {
  createContext,
  useContext,
  useReducer,
  useCallback,
  ReactNode,
} from "react";

export interface WizardState {
  file: File | null;
  contentHash: string | null;
  hashProgress: number;
  isHashing: boolean;
  manifest: Record<string, unknown> | null;
  advancedContentHash: string | null;
  advancedManifestHash: string | null;
  encryptionEnabled: boolean;
  spvResult: Record<string, unknown> | null;
}

export type WizardAction =
  | { type: "SET_FILE"; payload: File | null }
  | { type: "SET_CONTENT_HASH"; payload: string | null }
  | { type: "SET_HASH_PROGRESS"; payload: number }
  | { type: "SET_IS_HASHING"; payload: boolean }
  | { type: "SET_MANIFEST"; payload: Record<string, unknown> | null }
  | { type: "SET_ADVANCED_CONTENT_HASH"; payload: string | null }
  | { type: "SET_ADVANCED_MANIFEST_HASH"; payload: string | null }
  | { type: "SET_ENCRYPTION_ENABLED"; payload: boolean }
  | { type: "SET_SPV_RESULT"; payload: Record<string, unknown> | null }
  | { type: "RESET" };

const initialState: WizardState = {
  file: null,
  contentHash: null,
  hashProgress: 0,
  isHashing: false,
  manifest: null,
  advancedContentHash: null,
  advancedManifestHash: null,
  encryptionEnabled: false,
  spvResult: null,
};

function wizardReducer(state: WizardState, action: WizardAction): WizardState {
  switch (action.type) {
    case "SET_FILE":
      return { ...state, file: action.payload };
    case "SET_CONTENT_HASH":
      return { ...state, contentHash: action.payload };
    case "SET_HASH_PROGRESS":
      return { ...state, hashProgress: action.payload };
    case "SET_IS_HASHING":
      return { ...state, isHashing: action.payload };
    case "SET_MANIFEST":
      return { ...state, manifest: action.payload };
    case "SET_ADVANCED_CONTENT_HASH":
      return { ...state, advancedContentHash: action.payload };
    case "SET_ADVANCED_MANIFEST_HASH":
      return { ...state, advancedManifestHash: action.payload };
    case "SET_ENCRYPTION_ENABLED":
      return { ...state, encryptionEnabled: action.payload };
    case "SET_SPV_RESULT":
      return { ...state, spvResult: action.payload };
    case "RESET":
      return initialState;
    default:
      return state;
  }
}

interface WizardContextValue extends WizardState {
  setFile: (file: File | null) => void;
  setContentHash: (hash: string | null) => void;
  setHashProgress: (progress: number) => void;
  setIsHashing: (isHashing: boolean) => void;
  setManifest: (manifest: Record<string, unknown> | null) => void;
  setAdvancedContentHash: (hash: string | null) => void;
  setAdvancedManifestHash: (hash: string | null) => void;
  setEncryptionEnabled: (enabled: boolean) => void;
  setSPVResult: (result: Record<string, unknown> | null) => void;
  reset: () => void;
}

const WizardContext = createContext<WizardContextValue | undefined>(undefined);

export function WizardProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(wizardReducer, initialState);

  const setFile = useCallback((file: File | null) => {
    dispatch({ type: "SET_FILE", payload: file });
  }, []);

  const setContentHash = useCallback((hash: string | null) => {
    dispatch({ type: "SET_CONTENT_HASH", payload: hash });
  }, []);

  const setHashProgress = useCallback((progress: number) => {
    dispatch({ type: "SET_HASH_PROGRESS", payload: progress });
  }, []);

  const setIsHashing = useCallback((isHashing: boolean) => {
    dispatch({ type: "SET_IS_HASHING", payload: isHashing });
  }, []);

  const setManifest = useCallback((manifest: Record<string, unknown> | null) => {
    dispatch({ type: "SET_MANIFEST", payload: manifest });
  }, []);

  const setAdvancedContentHash = useCallback((hash: string | null) => {
    dispatch({ type: "SET_ADVANCED_CONTENT_HASH", payload: hash });
  }, []);

  const setAdvancedManifestHash = useCallback((hash: string | null) => {
    dispatch({ type: "SET_ADVANCED_MANIFEST_HASH", payload: hash });
  }, []);

  const setEncryptionEnabled = useCallback((enabled: boolean) => {
    dispatch({ type: "SET_ENCRYPTION_ENABLED", payload: enabled });
  }, []);

  const setSPVResult = useCallback((result: Record<string, unknown> | null) => {
    dispatch({ type: "SET_SPV_RESULT", payload: result });
  }, []);

  const reset = useCallback(() => {
    dispatch({ type: "RESET" });
  }, []);

  const value: WizardContextValue = {
    ...state,
    setFile,
    setContentHash,
    setHashProgress,
    setIsHashing,
    setManifest,
    setAdvancedContentHash,
    setAdvancedManifestHash,
    setEncryptionEnabled,
    setSPVResult,
    reset,
  };

  return (
    <WizardContext.Provider value={value}>{children}</WizardContext.Provider>
  );
}

export function useWizard(): WizardContextValue {
  const context = useContext(WizardContext);
  if (!context) {
    throw new Error("useWizard must be used within a WizardProvider");
  }
  return context;
}
