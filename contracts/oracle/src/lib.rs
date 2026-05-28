#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    vec, Bytes, BytesN, Env, Symbol, Address,
};

const REQUEST_TTL_LEDGERS: u32 = 100;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NotInitialized        = 1,
    UnauthorizedSigner    = 2,
    AlreadyInitialized    = 3,
    RegistryNotConfigured = 4,
    TeeNotVerified        = 5,
    ProviderNotRegistered = 6,
}

// ---------------------------------------------------------------------------
// Storage keys
// ---------------------------------------------------------------------------

#[contracttype]
pub enum DataKey {
    Registry,
    Provenance,
    Admin,
    Provider(Address),
    NextRequestId,
    Request(u64),
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestState {
    Pending,
    Verified,
    Rejected,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct VerificationRequest {
    pub storage_ref:   Bytes,
    pub manifest_hash: Bytes,
    pub requester:     Address,
    pub state:         RequestState,
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    pub fn init(
        env:        Env,
        registry:   Address,
        provenance: Address,
        admin:      Address,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Registry,   &registry);
        env.storage().instance().set(&DataKey::Provenance, &provenance);
        env.storage().instance().set(&DataKey::Admin,      &admin);
        Ok(())
    }

    pub fn add_provider(env: Env, provider: Address) -> Result<(), Error> {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Provider(provider), &true);
        Ok(())
    }

    pub fn remove_provider(env: Env, provider: Address) -> Result<(), Error> {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();
        env.storage().persistent().remove(&DataKey::Provider(provider));
        Ok(())
    }

    pub fn is_provider(env: Env, provider: Address) -> bool {
        env.storage().persistent()
            .get(&DataKey::Provider(provider))
            .unwrap_or(false)
    }

    pub fn submit_request(
        env:           Env,
        storage_ref:   Bytes,
        manifest_hash: Bytes,
        requester:     Address,
    ) -> u64 {
        requester.require_auth();

        let id: u64 = env.storage().instance()
            .get(&DataKey::NextRequestId)
            .unwrap_or(0u64)
            + 1;
        env.storage().instance().set(&DataKey::NextRequestId, &id);

        let req = VerificationRequest {
            storage_ref,
            manifest_hash,
            requester,
            state: RequestState::Pending,
        };

        env.storage().temporary().set(&DataKey::Request(id), &req);
        env.storage().temporary().extend_ttl(
            &DataKey::Request(id),
            REQUEST_TTL_LEDGERS,
            REQUEST_TTL_LEDGERS,
        );

        env.events().publish((Symbol::new(&env, "submitted"),), id);
        id
    }

    pub fn get_request(env: Env, id: u64) -> Option<VerificationRequest> {
        env.storage().temporary().get(&DataKey::Request(id))
    }

    pub fn verify_tee_hash(env: Env, tee_hash: BytesN<32>) -> Result<(), Error> {
        let registry: Address = env.storage().instance()
            .get(&DataKey::Registry)
            .ok_or(Error::RegistryNotConfigured)?;

        let approved: bool = env.invoke_contract(
            &registry,
            &Symbol::new(&env, "is_tee_hash_approved"),
            vec![&env, tee_hash.into()],
        );

        if !approved {
            return Err(Error::TeeNotVerified);
        }
        Ok(())
    }

    pub fn verify_attestation(
        env:       Env,
        provider:  BytesN<32>,
        tee_hash:  BytesN<32>,
        payload:   Bytes,
        signature: BytesN<64>,
    ) -> Result<(), Error> {
        let registry: Address = env.storage().instance()
            .get(&DataKey::Registry)
            .ok_or(Error::RegistryNotConfigured)?;

        let provider_ok: bool = env.invoke_contract(
            &registry,
            &Symbol::new(&env, "is_provider"),
            vec![&env, provider.clone().into()],
        );
        if !provider_ok {
            return Err(Error::UnauthorizedSigner);
        }

        let tee_ok: bool = env.invoke_contract(
            &registry,
            &Symbol::new(&env, "is_tee_hash_approved"),
            vec![&env, tee_hash.into()],
        );
        if !tee_ok {
            return Err(Error::TeeNotVerified);
        }

        env.crypto().ed25519_verify(&provider, &payload, &signature);

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, storage::Temporary as _},
        Env, Bytes, BytesN,
    };

    fn make_env() -> Env {
        Env::default()
    }

    fn register_oracle(env: &Env) -> Address {
        env.register_contract(None, OracleContract)
    }

    fn hash32(env: &Env, n: u8) -> BytesN<32> {
        BytesN::from_array(env, &[n; 32])
    }

    mod mock_registry {
        use soroban_sdk::{contract, contractimpl, BytesN, Env};

        #[contract]
        pub struct MockRegistry;

        #[contractimpl]
        impl MockRegistry {
            pub fn is_tee_hash_approved(env: Env, tee_hash: BytesN<32>) -> bool {
                tee_hash == BytesN::from_array(&env, &[1u8; 32])
            }

            pub fn is_provider(_env: Env, _provider: BytesN<32>) -> bool {
                true
            }
        }
    }

    mod reject_registry {
        use soroban_sdk::{contract, contractimpl, BytesN, Env};

        #[contract]
        pub struct RejectRegistry;

        #[contractimpl]
        impl RejectRegistry {
            pub fn is_provider(_env: Env, _provider: BytesN<32>) -> bool {
                false
            }
            pub fn is_tee_hash_approved(_env: Env, _tee_hash: BytesN<32>) -> bool {
                true
            }
        }
    }

    fn setup_with_mock_registry(env: &Env) -> (Address, Address) {
        let registry_id = env.register_contract(None, mock_registry::MockRegistry);
        let oracle_id   = register_oracle(env);
        let provenance  = Address::generate(env);
        let admin       = Address::generate(env);
        OracleContractClient::new(env, &oracle_id)
            .init(&registry_id, &provenance, &admin);
        (oracle_id, registry_id)
    }

    #[test]
    fn test_init() {
        let env = make_env();
        let cid = register_oracle(&env);
        let client = OracleContractClient::new(&env, &cid);

        let registry   = Address::generate(&env);
        let provenance = Address::generate(&env);
        let admin      = Address::generate(&env);

        client.init(&registry, &provenance, &admin);

        let result = client.try_init(&registry, &provenance, &admin);
        assert!(result.is_err());
    }

    #[test]
    fn test_init_already_initialized() {
        let env = make_env();
        let cid = register_oracle(&env);
        let client = OracleContractClient::new(&env, &cid);

        let registry   = Address::generate(&env);
        let provenance = Address::generate(&env);
        let admin      = Address::generate(&env);

        client.init(&registry, &provenance, &admin);

        let err = client
            .try_init(&registry, &provenance, &admin)
            .unwrap_err()
            .unwrap();
        assert_eq!(err, Error::AlreadyInitialized);
    }

    #[test]
    fn test_provider_management() {
        let env = make_env();
        env.mock_all_auths();
        let cid = register_oracle(&env);
        let client = OracleContractClient::new(&env, &cid);

        let registry   = Address::generate(&env);
        let provenance = Address::generate(&env);
        let admin      = Address::generate(&env);
        let provider   = Address::generate(&env);

        client.init(&registry, &provenance, &admin);

        assert!(!client.is_provider(&provider));
        client.add_provider(&provider);
        assert!(client.is_provider(&provider));
        client.remove_provider(&provider);
        assert!(!client.is_provider(&provider));
    }

    #[test]
    fn test_submit_request_generates_unique_ids() {
        let env = make_env();
        env.mock_all_auths();
        let cid    = register_oracle(&env);
        let client = OracleContractClient::new(&env, &cid);
        let bytes  = Bytes::from_slice(&env, b"data");
        let req    = Address::generate(&env);

        assert_eq!(client.submit_request(&bytes, &bytes, &req), 1);
        assert_eq!(client.submit_request(&bytes, &bytes, &req), 2);
        assert_eq!(client.submit_request(&bytes, &bytes, &req), 3);
    }

    #[test]
    fn test_submit_request_stores_pending_in_temporary_storage() {
        let env = make_env();
        env.mock_all_auths();
        let cid    = register_oracle(&env);
        let client = OracleContractClient::new(&env, &cid);
        let bytes  = Bytes::from_slice(&env, b"ref");
        let req    = Address::generate(&env);

        let id = client.submit_request(&bytes, &bytes, &req);
        assert_eq!(client.get_request(&id).unwrap().state, RequestState::Pending);

        let ttl = env.as_contract(&cid, || {
            env.storage().temporary().get_ttl(&DataKey::Request(id))
        });
        assert!(ttl > 0);
    }

    #[test]
    fn test_request_state_pending_on_submit() {
        let env = make_env();
        env.mock_all_auths();
        let cid    = register_oracle(&env);
        let client = OracleContractClient::new(&env, &cid);
        let bytes  = Bytes::from_slice(&env, b"manifest");
        let req    = Address::generate(&env);

        let id     = client.submit_request(&bytes, &bytes, &req);
        let stored = client.get_request(&id).unwrap();
        assert_eq!(stored.state, RequestState::Pending);
        assert_eq!(stored.manifest_hash, bytes);
    }

    #[test]
    fn test_verify_tee_hash_success() {
        let env = make_env();
        env.mock_all_auths();
        let (oracle_id, _) = setup_with_mock_registry(&env);
        let client = OracleContractClient::new(&env, &oracle_id);

        client.verify_tee_hash(&BytesN::from_array(&env, &[1u8; 32]));
    }

    #[test]
    fn test_verify_tee_hash_invalid_hash() {
        let env = make_env();
        env.mock_all_auths();
        let (oracle_id, _) = setup_with_mock_registry(&env);
        let client = OracleContractClient::new(&env, &oracle_id);

        let err = client
            .try_verify_tee_hash(&BytesN::from_array(&env, &[0u8; 32]))
            .unwrap_err()
            .unwrap();
        assert_eq!(err, Error::TeeNotVerified);
    }

    #[test]
    fn test_verify_tee_hash_registry_call_failure() {
        let env = make_env();
        let cid    = register_oracle(&env);
        let client = OracleContractClient::new(&env, &cid);

        let err = client
            .try_verify_tee_hash(&BytesN::from_array(&env, &[1u8; 32]))
            .unwrap_err()
            .unwrap();
        assert_eq!(err, Error::RegistryNotConfigured);
    }

    #[test]
    fn test_verify_attestation_not_initialized() {
        let env = make_env();
        let cid    = register_oracle(&env);
        let client = OracleContractClient::new(&env, &cid);

        let err = client
            .try_verify_attestation(
                &hash32(&env, 0),
                &hash32(&env, 1),
                &Bytes::from_slice(&env, b"data"),
                &BytesN::from_array(&env, &[0u8; 64]),
            )
            .unwrap_err()
            .unwrap();
        assert_eq!(err, Error::RegistryNotConfigured);
    }

    #[test]
    fn test_verify_attestation_unauthorized_signer() {
        let env = make_env();
        env.mock_all_auths();
        let registry_id = env.register_contract(None, reject_registry::RejectRegistry);
        let oracle_id   = register_oracle(&env);
        let provenance  = Address::generate(&env);
        let admin       = Address::generate(&env);
        OracleContractClient::new(&env, &oracle_id)
            .init(&registry_id, &provenance, &admin);
        let client = OracleContractClient::new(&env, &oracle_id);

        let err = client
            .try_verify_attestation(
                &hash32(&env, 0),
                &hash32(&env, 1),
                &Bytes::from_slice(&env, b"data"),
                &BytesN::from_array(&env, &[0u8; 64]),
            )
            .unwrap_err()
            .unwrap();
        assert_eq!(err, Error::UnauthorizedSigner);
    }

    #[test]
    fn test_verify_attestation_invalid_signature() {
        let env = make_env();
        env.mock_all_auths();
        let (oracle_id, _) = setup_with_mock_registry(&env);
        let client = OracleContractClient::new(&env, &oracle_id);

        let sk       = ed25519_dalek::SigningKey::from_bytes(&[42u8; 32]);
        let provider = BytesN::from_array(&env, sk.verifying_key().as_bytes());
        let tee_hash = BytesN::from_array(&env, &[1u8; 32]);
        let payload  = Bytes::from_slice(&env, b"payload");
        let bad_sig  = BytesN::from_array(&env, &[0u8; 64]);

        let result = client.try_verify_attestation(&provider, &tee_hash, &payload, &bad_sig);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_attestation_success() {
        use ed25519_dalek::Signer;

        let env = make_env();
        env.mock_all_auths();
        let (oracle_id, _) = setup_with_mock_registry(&env);
        let client = OracleContractClient::new(&env, &oracle_id);

        let sk       = ed25519_dalek::SigningKey::from_bytes(&[42u8; 32]);
        let provider = BytesN::from_array(&env, sk.verifying_key().as_bytes());
        let tee_hash = BytesN::from_array(&env, &[1u8; 32]);
        let raw      = b"attestation payload";
        let payload  = Bytes::from_slice(&env, raw);

        let sig: ed25519_dalek::Signature = sk.sign(raw);
        let signature = BytesN::from_array(&env, &sig.to_bytes());

        client.verify_attestation(&provider, &tee_hash, &payload, &signature);
    }
}
