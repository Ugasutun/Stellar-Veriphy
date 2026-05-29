#[cfg(test)]
mod tests {
    use crate::{ProvenanceContract, ProvenanceContractClient, ProvenanceError};
    use soroban_sdk::{testutils::Address as _, Env, String};

    fn s(env: &Env, v: &str) -> String {
        String::from_str(env, v)
    }

    // --- Issue #38 ---

    #[test]
    fn test_double_initialization() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = soroban_sdk::Address::generate(&env);
        client.initialize(&oracle);
        assert!(client.try_initialize(&oracle).is_err());
    }

    #[test]
    fn test_mint_without_initialization() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let to = soroban_sdk::Address::generate(&env);
        assert!(client
            .try_mint(&s(&env, "sid"), &s(&env, "mhash"), &s(&env, "ahash"), &to)
            .is_err());
    }

    // --- Issue #39 ---

    #[test]
    fn test_mint_certificate() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = soroban_sdk::Address::generate(&env);
        let to = soroban_sdk::Address::generate(&env);
        client.initialize(&oracle);

        let id = client.mint(&s(&env, "storage_ref"), &s(&env, "manifest_hash"), &s(&env, "attestation_hash"), &to);
        assert_eq!(id, 1);

        let cert = client.get_certificate(&id).unwrap();
        assert_eq!(cert.storage_ref, s(&env, "storage_ref"));
        assert_eq!(cert.manifest_hash, s(&env, "manifest_hash"));
        assert_eq!(cert.attestation_hash, s(&env, "attestation_hash"));
        assert_eq!(cert.creator, to);
    }

    #[test]
    fn test_mint_multiple_certificates() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = soroban_sdk::Address::generate(&env);
        let to = soroban_sdk::Address::generate(&env);
        client.initialize(&oracle);
        assert_eq!(client.mint(&s(&env, "sid"), &s(&env, "mh1"), &s(&env, "ah"), &to), 1);
        assert_eq!(client.mint(&s(&env, "sid"), &s(&env, "mh2"), &s(&env, "ah"), &to), 2);
        assert_eq!(client.mint(&s(&env, "sid"), &s(&env, "mh3"), &s(&env, "ah"), &to), 3);
    }

    #[test]
    fn test_get_nonexistent_certificate() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        assert_eq!(
            client.try_get_certificate(&999u64).unwrap_err().unwrap(),
            ProvenanceError::CertificateNotFound
        );
    }

    // --- Issue #40 ---

    #[test]
    fn test_prevent_duplicate_manifest_hash() {
        let env = Env::default();
        env.mock_all_auths();
        let cid = env.register_contract(None, ProvenanceContract);
        let client = ProvenanceContractClient::new(&env, &cid);
        let oracle = soroban_sdk::Address::generate(&env);
        let to = soroban_sdk::Address::generate(&env);
        client.initialize(&oracle);
        client.mint(&s(&env, "sid"), &s(&env, "mhash"), &s(&env, "ahash"), &to);
        assert!(client
            .try_mint(&s(&env, "sid"), &s(&env, "mhash"), &s(&env, "ahash"), &to)
            .is_err());
    }
}
