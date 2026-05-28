#![no_std]
use soroban_sdk::{contract, contractimpl, BytesN, Env, Symbol};

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
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
