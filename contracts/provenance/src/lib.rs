#![no_std]
use soroban_sdk::{
    contract, contractevent, contracterror, contractimpl, contracttype,
    symbol_short, Address, Env, String,
};

// #16 — typed error enum
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProvenanceError {
    CertificateNotFound = 1,
}

// #14 — String fields (was Bytes)
#[contracttype]
pub struct ProvenanceCert {
    pub storage_ref:      String,
    pub manifest_hash:    String,
    pub attestation_hash: String,
    pub creator:          Address,
    pub timestamp:        u64,
}

// #15 — typed contract event
#[contractevent]
pub struct CertificateMinted {
    #[topic]
    pub owner: Address,
    #[topic]
    pub certificate_id: u64,
    #[topic]
    pub manifest_hash: String,
}

#[contract]
pub struct ProvenanceContract;

#[contractimpl]
impl ProvenanceContract {
    /// One-time setup — stores the oracle address authorised to mint.
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
        storage_ref:      String,
        manifest_hash:    String,
        attestation_hash: String,
        to:               Address,
    ) -> u64 {
        let oracle: Address = env
            .storage()
            .persistent()
            .get(&symbol_short!("ORACLE"))
            .expect("Not initialized");
        oracle.require_auth();

        // #13 — duplicate prevention
        let mani_key = (symbol_short!("MANI"), manifest_hash.clone());
        if env.storage().persistent().has(&mani_key) {
            panic!("Certificate already exists for this manifest hash");
        }

        let cnt_key = symbol_short!("CERT_CNT");
        let id: u64 = env
            .storage()
            .persistent()
            .get(&cnt_key)
            .unwrap_or(0u64)
            + 1;
        env.storage().persistent().set(&cnt_key, &id);

        let cert = ProvenanceCert {
            storage_ref,
            manifest_hash: manifest_hash.clone(),
            attestation_hash,
            creator: to.clone(),
            timestamp: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&id, &cert);

        // #13 — store manifest → id mapping
        env.storage().persistent().set(&mani_key, &id);

        // #15 — emit typed event
        CertificateMinted {
            owner: to,
            certificate_id: id,
            manifest_hash,
        }
        .emit(&env);

        id
    }

    // #16 — returns Result instead of panicking
    pub fn get_certificate(
        env: Env,
        id:  u64,
    ) -> Result<ProvenanceCert, ProvenanceError> {
        env.storage()
            .persistent()
            .get(&id)
            .ok_or(ProvenanceError::CertificateNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    fn s(env: &Env, v: &str) -> String {
        String::from_str(env, v)
    }

    #[test]
    fn test_double_initialization() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = Address::generate(&env);
        client.initialize(&oracle);
        assert!(client.try_initialize(&oracle).is_err());
    }

    #[test]
    fn test_mint_without_initialization() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let to = Address::generate(&env);
        assert!(client
            .try_mint(&s(&env, "sid"), &s(&env, "mhash"), &s(&env, "ahash"), &to)
            .is_err());
    }

    #[test]
    fn test_mint_multiple_certificates() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = Address::generate(&env);
        let to = Address::generate(&env);
        client.initialize(&oracle);
        assert_eq!(client.mint(&s(&env, "sid"), &s(&env, "mh1"), &s(&env, "ah"), &to), 1);
        assert_eq!(client.mint(&s(&env, "sid"), &s(&env, "mh2"), &s(&env, "ah"), &to), 2);
        assert_eq!(client.mint(&s(&env, "sid"), &s(&env, "mh3"), &s(&env, "ah"), &to), 3);
    }

    // #13
    #[test]
    fn test_prevent_duplicate_manifest_hash() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = Address::generate(&env);
        let to = Address::generate(&env);
        client.initialize(&oracle);
        client.mint(&s(&env, "sid"), &s(&env, "mhash"), &s(&env, "ahash"), &to);
        assert!(client
            .try_mint(&s(&env, "sid"), &s(&env, "mhash"), &s(&env, "ahash"), &to)
            .is_err());
    }

    // #15
    #[test]
    fn test_certificate_minted_event() {
        use soroban_sdk::testutils::Events as _;
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = Address::generate(&env);
        let to = Address::generate(&env);
        client.initialize(&oracle);
        client.mint(&s(&env, "sid"), &s(&env, "mhash"), &s(&env, "ahash"), &to);
        let events = env.events().all();
        assert!(!events.is_empty());
    }

    // #16
    #[test]
    fn test_get_nonexistent_certificate() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        assert!(client.try_get_certificate(&999u64).is_err());
    }
}
