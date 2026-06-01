export interface ContentManifest {
  contentHash: string;       // sha256 of the media file
  creator: string;           // Stellar public key (G...)
  timestamp: string;         // ISO 8601
  metadata?: {
    device?: string;
    location?: string;
    aiModel?: string;
  };
}

export interface ProvenanceCert {
  id: string;
  storageRef: string;
  manifestHash: string;
  attestationHash: string;
  creator: string;
  timestamp: number;
}

export type VerificationStatus = "pending" | "processing" | "certified" | "failed";

// ---------------------------------------------------------------------------
// Verification mode
// ---------------------------------------------------------------------------

/** Which verification path the user has chosen. */
export type VerificationMode = "standard" | "advanced";

// ---------------------------------------------------------------------------
// Wallet connection status
// ---------------------------------------------------------------------------

/** Current state of the Freighter wallet connection. */
export type WalletConnectionStatus = "disconnected" | "connecting" | "connected";

// ---------------------------------------------------------------------------
// CertificateDetails — mirrors the on-chain ProvenanceCert struct
// ---------------------------------------------------------------------------

/**
 * Frontend representation of a minted provenance certificate.
 * Field names are camelCase equivalents of the Soroban `ProvenanceCert` struct.
 */
export interface CertificateDetails {
  /** Auto-incrementing on-chain certificate identifier (u64 on-chain). */
  id: string;
  /** IPFS / Arweave storage reference for the original media file. */
  storageRef: string;
  /** SHA-256 hex digest of the manifest JSON. */
  manifestHash: string;
  /** SHA-256 hex digest of the TEE attestation payload. */
  attestationHash: string;
  /** Stellar public key of the content creator. */
  creator: string;
  /** Ledger timestamp (seconds since Unix epoch) at the time of minting. */
  timestamp: number;
}

// ---------------------------------------------------------------------------
// VerificationJob
// ---------------------------------------------------------------------------

/** Lifecycle status of a verification job submitted to the Oracle contract. */
export type VerificationJobStatus = "pending" | "processing" | "verified" | "rejected" | "failed";

/**
 * Tracks a single verification job from submission through to certificate
 * issuance (or failure).
 */
export interface VerificationJob {
  /** Unique job identifier returned by the Oracle `submit_request` call. */
  jobId: string;
  /** Current lifecycle status of the job. */
  status: VerificationJobStatus;
  /** SHA-256 hex digest of the media content being verified. */
  contentHash: string;
  /** SHA-256 hex digest of the attached manifest JSON. */
  manifestHash: string;
  /** On-chain certificate ID, populated once the job reaches `verified` status. */
  certificateId?: string;
}

// ---------------------------------------------------------------------------
// ApiResponse — generic wrapper for all API / service responses
// ---------------------------------------------------------------------------

/**
 * Generic wrapper returned by service functions and API routes.
 *
 * On success: `{ success: true, data: T }`
 * On failure: `{ success: false, error: string }`
 */
export type ApiResponse<T> =
  | { success: true; data: T; error?: never }
  | { success: false; error: string; data?: never };
