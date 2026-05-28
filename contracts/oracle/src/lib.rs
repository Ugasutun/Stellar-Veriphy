#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Bytes, Env, Symbol, Address};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const REQUEST_TTL_LEDGERS: u32 = 100;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized     = 2,
    NotFound           = 3,
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
    /// One-time initialisation. Stores registry, provenance, and admin addresses.
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

    /// Submit a new verification request. Returns the unique request ID.
    pub fn submit_request(
        env:          Env,
        storage_ref:  Bytes,
        manifest_hash: Bytes,
        requester:    Address,
    ) -> u64 {
        requester.require_auth();

        // Monotonic counter
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

        // Temporary storage with TTL
        env.storage().temporary().set(&DataKey::Request(id), &req);
        env.storage().temporary().extend_ttl(&DataKey::Request(id), REQUEST_TTL_LEDGERS, REQUEST_TTL_LEDGERS);

        env.events().publish((Symbol::new(&env, "submitted"),), id);
        id
    }

    /// Retrieve a verification request by ID.
    pub fn get_request(env: Env, id: u64) -> Option<VerificationRequest> {
        env.storage().temporary().get(&DataKey::Request(id))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::{Address as _}, Env, Bytes};
    use soroban_sdk::testutils::storage::Temporary as TemporaryStorage;

    fn make_env() -> Env {
        Env::default()
    }

    fn register(env: &Env) -> soroban_sdk::Address {
        env.register_contract(None, OracleContract)
    }

    // Issue #1 tests
    #[test]
    fn test_init() {
        let env = make_env();
        let contract_id = register(&env);
        let client = OracleContractClient::new(&env, &contract_id);

        let registry   = Address::generate(&env);
        let provenance = Address::generate(&env);
        let admin      = Address::generate(&env);

        client.init(&registry, &provenance, &admin);

        // Second call must fail
        let result = client.try_init(&registry, &provenance, &admin);
        assert!(result.is_err());
    }

    #[test]
    fn test_init_already_initialized() {
        let env = make_env();
        let contract_id = register(&env);
        let client = OracleContractClient::new(&env, &contract_id);

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

    // Issue #2 test
    #[test]
    fn test_submit_request_generates_unique_ids() {
        let env = make_env();
        env.mock_all_auths();
        let contract_id = register(&env);
        let client = OracleContractClient::new(&env, &contract_id);

        let requester = Address::generate(&env);
        let bytes     = Bytes::from_slice(&env, b"data");

        let id1 = client.submit_request(&bytes, &bytes, &requester);
        let id2 = client.submit_request(&bytes, &bytes, &requester);
        let id3 = client.submit_request(&bytes, &bytes, &requester);

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    // Issue #3 test
    #[test]
    fn test_submit_request_stores_pending_in_temporary_storage() {
        let env = make_env();
        env.mock_all_auths();
        let contract_id = register(&env);
        let client = OracleContractClient::new(&env, &contract_id);

        let requester = Address::generate(&env);
        let bytes     = Bytes::from_slice(&env, b"ref");

        let id = client.submit_request(&bytes, &bytes, &requester);

        let req = client.get_request(&id).unwrap();
        assert_eq!(req.state, RequestState::Pending);

        // TTL should be positive
        let ttl = env.as_contract(&contract_id, || {
            env.storage().temporary().get_ttl(&DataKey::Request(id))
        });
        assert!(ttl > 0);
    }

    // Issue #4 test — RequestState enum and state field
    #[test]
    fn test_request_state_pending_on_submit() {
        let env = make_env();
        env.mock_all_auths();
        let contract_id = register(&env);
        let client = OracleContractClient::new(&env, &contract_id);

        let requester = Address::generate(&env);
        let bytes     = Bytes::from_slice(&env, b"manifest");

        let id  = client.submit_request(&bytes, &bytes, &requester);
        let req = client.get_request(&id).unwrap();

        assert_eq!(req.state, RequestState::Pending);
    }
}
