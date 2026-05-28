#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env, Symbol, Address};

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    /// Add a verified provider.
    pub fn add_provider(env: Env, admin: Address, provider: BytesN<32>) -> Result<(), VerificationError> {
        // Check that the caller is the admin.
        let invoker = env.invoker();
        if invoker != admin {
            return Err(VerificationError::Unauthorized);
        }
        // Store the provider.
        let storage_key = b"providers";
        let providers_map = env.storage().persistent().get::<Symbol, BytesN<32>>(storage_key).unwrap_or(BytesN::from_array([0u8; 32]));
        // We'll use a set: we can store a map with provider as key and value as ().
        // For simplicity, we'll store a vector? We'll use a map.
        // We'll create a map keyed by provider.
        let providers = env.storage().persistent().get::<BytesN<32>, ()>(&provider);
        if providers.is_some() {
            // Already exists, we can still return Ok or error? We'll return Ok for idempotency.
            // But the spec doesn't specify, so we'll just store again (no-op).
        }
        env.storage().persistent().set(&provider, &());
        // Emit event.
        let event_data = ProviderEventData { provider };
        env.events().publish((Symbol::new(&env, "provider_added"),), event_data);
        Ok(())
    }

    /// Add a verified TEE code hash.
    pub fn add_tee_hash(env: Env, admin: Address, code_hash: BytesN<32>) -> Result<(), VerificationError> {
        // Check that the caller is the admin.
        let invoker = env.invoker();
        if invoker != admin {
            return Err(VerificationError::Unauthorized);
        }
        // Check for duplicate.
        let existing = env.storage().persistent().get::<BytesN<32>, ()>(&code_hash);
        if existing.is_some() {
            return Err(VerificationError::DuplicateHash);
        }
        // Store the tee hash.
        env.storage().persistent().set(&code_hash, &());
        // Emit event.
        let event_data = TeeHashEventData { hash: code_hash };
        env.events().publish((Symbol::new(&env, "tee_hash_added"),), event_data);
        Ok(())
    }

    /// Process a verification attestation.
    pub fn process_verification(
        env: Env,
        attestation: Attestation,
        signature: BytesN<64>,
    ) -> Result<(), VerificationError> {
        // 1. Check that the provider is registered.
        let provider_key = &attestation.provider;
        let provider_exists = env.storage().persistent().get::<BytesN<32>, ()>(provider_key).is_some();
        if !provider_exists {
            return Err(VerificationError::NotFound);
        }

        // 2. Check that the TEE hash is registered.
        let tee_hash_key = &attestation.tee_hash;
        let tee_hash_exists = env.storage().persistent().get::<BytesN<32>, ()>(tee_hash_key).is_some();
        if !tee_hash_exists {
            return Err(VerificationError::InvalidTeeHash);
        }

        // 3. Check if already processed.
        let mut processed_key = [0u8; 72]; // 32 + 32 + 8
        processed_key[..32].copy_from_slice(&attestation.provider.0);
        processed_key[32..64].copy_from_slice(&attestation.tee_hash.0);
        processed_key[64..].copy_from_slice(&attestation.request_id.to_be_bytes());
        let processed_key_bytes = Bytes::from_slice(&env, &processed_key);
        let processed = env.storage().persistent().get::<Bytes, ()>(&processed_key_bytes).is_some();
        if processed {
            return Err(VerificationError::AlreadyProcessed);
        }

        // 4. Serialize the attestation to XDR (simplified as concatenation).
        let mut attestation_bytes = Vec::with_capacity(32 + 32 + 8);
        attestation_bytes.extend_from_slice(&attestation.provider.0);
        attestation_bytes.extend_from_slice(&attestation.tee_hash.0);
        attestation_bytes.extend_from_slice(&attestation.request_id.to_be_bytes());

        // 5. Verify the Ed25519 signature.
        let valid = env.crypto().ed25519_verify(
            &attestation_bytes,
            &signature,
            &attestation.provider, // assuming provider is the public key
        );
        if !valid {
            return Err(VerificationError::InvalidSignature);
        }

        // 6. Mark as processed.
        env.storage().persistent().set(&processed_key_bytes, &());
        Ok(())
    }

    /// Check if a TEE code hash is approved (existing function).
    pub fn is_approved(env: Env, code_hash: Bytes) -> bool {
        env.storage().persistent().get(&code_hash).unwrap_or(false)
    /// Register an approved TEE code hash (admin-gated).
    pub fn register_tee_hash(env: Env, admin: soroban_sdk::Address, tee_hash: BytesN<32>) {
        admin.require_auth();
        env.storage().persistent().set(&tee_hash, &true);
        env.events().publish((Symbol::new(&env, "tee_registered"),), tee_hash);
    }

    /// Check if a TEE code hash is approved.
    pub fn is_tee_hash_approved(env: Env, tee_hash: BytesN<32>) -> bool {
        env.storage().persistent().get(&tee_hash).unwrap_or(false)
    }

    /// Register a provider public key (admin-gated).
    pub fn register_provider(env: Env, admin: soroban_sdk::Address, provider: BytesN<32>) {
        admin.require_auth();
        env.storage().persistent().set(&provider, &true);
        env.events().publish((Symbol::new(&env, "provider_registered"),), provider);
    }

    /// Check if a provider public key is registered.
    pub fn is_provider(env: Env, provider: BytesN<32>) -> bool {
        env.storage().persistent().get(&provider).unwrap_or(false)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracttype]
pub enum VerificationError {
    NotFound = 1,
    Unauthorized = 2,
    InvalidSignature = 3,
    InvalidAttestation = 4,
    AlreadyProcessed = 5,
    InvalidTeeHash = 6,
    DuplicateHash = 7,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracttype]
pub struct ProviderEventData {
    pub provider: BytesN<32>,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracttype]
pub struct TeeHashEventData {
    pub hash: BytesN<32>,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracttype]
pub struct Attestation {
    pub provider: BytesN<32>,
    pub tee_hash: BytesN<32>,
    pub request_id: u64,
}