"use client";

import { createContext, useContext, useState, useCallback } from "react";

export type WizardStep = "ModeSelection" | "MediaInput" | "ManifestStep" | "SPVOptions" | "SPVResults";

interface WizardContextValue {
  currentStep: number;
  steps: WizardStep[];
  goToStep: (step: number) => void;
  nextStep: () => void;
  prevStep: () => void;
  reset: () => void;
  isStepComplete: (step: number) => boolean;
  setStepComplete: (step: number, complete: boolean) => void;
}

const WizardContext = createContext<WizardContextValue | undefined>(undefined);

const STEPS: WizardStep[] = ["ModeSelection", "MediaInput", "ManifestStep", "SPVOptions", "SPVResults"];

export function WizardProvider({ children }: { children: React.ReactNode }) {
  const [currentStep, setCurrentStep] = useState(0);
  const [completedSteps, setCompletedSteps] = useState<Set<number>>(new Set());

  const goToStep = useCallback((step: number) => {
    if (step >= 0 && step < STEPS.length) {
      setCurrentStep(step);
    }
  }, []);

  const nextStep = useCallback(() => {
    setCurrentStep((prev) => (prev < STEPS.length - 1 ? prev + 1 : prev));
  }, []);

  const prevStep = useCallback(() => {
    setCurrentStep((prev) => (prev > 0 ? prev - 1 : prev));
  }, []);

  const reset = useCallback(() => {
    setCurrentStep(0);
    setCompletedSteps(new Set());
  }, []);

  const isStepComplete = useCallback((step: number) => {
    return completedSteps.has(step);
  }, [completedSteps]);

  const setStepComplete = useCallback((step: number, complete: boolean) => {
    setCompletedSteps((prev) => {
      const next = new Set(prev);
      if (complete) {
        next.add(step);
      } else {
        next.delete(step);
      }
      return next;
    });
  }, []);

  return (
    <WizardContext.Provider
      value={{
        currentStep,
        steps: STEPS,
        goToStep,
        nextStep,
        prevStep,
        reset,
        isStepComplete,
        setStepComplete,
      }}
    >
      {children}
    </WizardContext.Provider>
  );
}

export const useWizard = () => {
  const context = useContext(WizardContext);
  if (!context) {
    throw new Error("useWizard must be used within WizardProvider");
  }
  return context;
};
