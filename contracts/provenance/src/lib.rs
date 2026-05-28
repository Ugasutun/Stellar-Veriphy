#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol};

#[contracttype]
pub struct CertificateDetails {
    pub storage_id: String,
    pub manifest_hash: String,
    pub attestation_hash: String,
}

#[contracttype]
pub struct Certificate {
    pub storage_id: String,
    pub manifest_hash: String,
    pub attestation_hash: String,
    pub creator: Address,
    /// Ledger timestamp at mint time; set once and immutable (no update API).
    pub timestamp: u64,
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Bytes, Env, Symbol, Address};

#[contracttype]
pub struct ProvenanceCert {
    pub storage_ref:      Bytes,
    pub manifest_hash:    Bytes,
    pub attestation_hash: Bytes,
    pub creator:          Address,
    pub timestamp:        u64,
}

#[contract]
pub struct ProvenanceContract;

#[contractimpl]
impl ProvenanceContract {
    pub fn mint(env: Env, to: Address, details: CertificateDetails) -> u64 {
        to.require_auth();
        let id: u64 = env.ledger().sequence() as u64;
        let cert = Certificate {
            storage_id: details.storage_id,
            manifest_hash: details.manifest_hash,
            attestation_hash: details.attestation_hash,
    /// One-time setup — stores the oracle address that is authorised to mint.
    pub fn initialize(env: Env, oracle: Address) {
        let key = symbol_short!("ORACLE");
        if env.storage().persistent().has(&key) {
            panic!("Contract already initialized");
        }
        env.storage().persistent().set(&key, &oracle);
    }

    /// Mint a provenance certificate. Only the oracle may call this.
    pub fn mint(
        env:              Env,
        storage_ref:      Bytes,
        manifest_hash:    Bytes,
        attestation_hash: Bytes,
        to:               Address,
    ) -> u64 {
        let oracle: Address = env.storage().persistent()
            .get(&symbol_short!("ORACLE"))
            .expect("Not initialized");
        oracle.require_auth();

        let cnt_key = symbol_short!("CERT_CNT");
        let id: u64 = env.storage().persistent()
            .get(&cnt_key)
            .unwrap_or(0u64)
            + 1;
        env.storage().persistent().set(&cnt_key, &id);

        let cert = ProvenanceCert {
            storage_ref,
            manifest_hash,
            attestation_hash,
            creator: to,
            timestamp: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&id, &cert);
        env.events().publish((Symbol::new(&env, "minted"),), id);
        id
    }

    pub fn get(env: Env, id: u64) -> Certificate {
        env.storage().persistent().get(&id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger as _},
        Env, String,
    };

    #[test]
    fn test_immutable_timestamp() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &contract_id);

        env.ledger().with_mut(|l| {
            l.timestamp = 100;
        });

        let details = CertificateDetails {
            storage_id: String::from_str(&env, "sid"),
            manifest_hash: String::from_str(&env, "mhash"),
            attestation_hash: String::from_str(&env, "ahash"),
        };
        let to = Address::generate(&env);
        let id = client.mint(&to, &details);
        let original_ts = client.get(&id).timestamp;

        env.ledger().with_mut(|l| {
            l.timestamp = 999;
        });

        assert_eq!(client.get(&id).timestamp, original_ts);
// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, Bytes};

    fn make_env() -> Env {
        Env::default()
    }

    fn register(env: &Env) -> Address {
        env.register_contract(None, ProvenanceContract)
    }

    fn b(env: &Env) -> Bytes {
        Bytes::from_slice(env, b"data")
    }

    #[test]
    fn test_double_initialization() {
        let env    = make_env();
        let cid    = register(&env);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = Address::generate(&env);

        client.initialize(&oracle);

        let result = client.try_initialize(&oracle);
        assert!(result.is_err());
    }

    #[test]
    fn test_mint_without_initialization() {
        let env = make_env();
        env.mock_all_auths();
        let cid    = register(&env);
        let client = ProvenanceContractClient::new(&env, &cid);
        let to     = Address::generate(&env);
        let data   = b(&env);

        let result = client.try_mint(&data, &data, &data, &to);
        assert!(result.is_err());
    }

    #[test]
    fn test_mint_multiple_certificates() {
        let env    = make_env();
        env.mock_all_auths();
        let cid    = register(&env);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = Address::generate(&env);
        let to     = Address::generate(&env);
        let data   = b(&env);

        client.initialize(&oracle);

        assert_eq!(client.mint(&data, &data, &data, &to), 1);
        assert_eq!(client.mint(&data, &data, &data, &to), 2);
        assert_eq!(client.mint(&data, &data, &data, &to), 3);
    }
}
