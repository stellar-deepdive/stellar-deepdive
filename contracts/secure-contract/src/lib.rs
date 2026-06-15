#![no_std]
use soroban_sdk::{
    contract, contractclient, contracterror, contractimpl, panic_with_error, symbol_short,
    Address, Env, Symbol,
};

/// Interface implemented by the access-control contract this contract delegates
/// permission checks to. Defined as a trait (rather than `contractimport!`) so
/// `SecureContract` can be built and tested without a prebuilt access-control wasm.
#[contractclient(name = "AccessControlClient")]
pub trait AccessControlInterface {
    fn check_permission(env: Env, user: Address, function: Symbol) -> bool;
}

/// Errors returned by `SecureContract`.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// `initialize` has not been called yet, so the ACL contract address is unknown.
    NotInitialized = 1,
    /// The caller does not have the permission required for this function.
    AccessDenied = 2,
}

#[contract]
pub struct SecureContract;

#[contractimpl]
impl SecureContract {
    pub fn initialize(env: Env, admin: Address, acl_contract: Address) {
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("acl"), &acl_contract);
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }

    pub fn protected_function(env: Env, caller: Address) -> u32 {
        caller.require_auth();
        Self::check_permission(&env, &caller, symbol_short!("protected"));
        42
    }

    pub fn admin_only(env: Env, caller: Address) -> bool {
        caller.require_auth();
        Self::check_permission(&env, &caller, symbol_short!("admin"));
        true
    }

    fn check_permission(env: &Env, user: &Address, function: Symbol) {
        let acl_addr: Address = match env.storage().instance().get(&symbol_short!("acl")) {
            Some(addr) => addr,
            None => panic_with_error!(env, ContractError::NotInitialized),
        };

        let acl_client = AccessControlClient::new(env, &acl_addr);

        if !acl_client.check_permission(user, &function) {
            panic_with_error!(env, ContractError::AccessDenied);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    /// Minimal stand-in for the access-control contract: grants permission for
    /// whichever function symbol it is constructed to allow.
    #[contract]
    struct MockAcl;

    #[contractimpl]
    impl MockAcl {
        pub fn check_permission(_env: Env, _user: Address, function: Symbol) -> bool {
            function == symbol_short!("protected")
        }
    }

    fn setup(env: &Env) -> (SecureContractClient<'_>, Address, Address) {
        let acl_id = env.register_contract(None, MockAcl);
        let secure_id = env.register_contract(None, SecureContract);
        let client = SecureContractClient::new(env, &secure_id);
        (client, acl_id, secure_id)
    }

    #[test]
    fn test_protected_function_with_permission() {
        let env = Env::default();
        let (client, acl_id, _) = setup(&env);

        let admin = Address::generate(&env);
        let caller = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin, &acl_id);

        assert_eq!(client.protected_function(&caller), 42);
    }

    #[test]
    fn test_admin_only_without_permission_returns_access_denied() {
        let env = Env::default();
        let (client, acl_id, _) = setup(&env);

        let admin = Address::generate(&env);
        let caller = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin, &acl_id);

        // MockAcl only grants "protected", so "admin" must be denied.
        let result = client.try_admin_only(&caller);
        assert_eq!(result, Err(Ok(ContractError::AccessDenied)));
    }

    #[test]
    fn test_call_before_initialize_returns_not_initialized() {
        let env = Env::default();
        let (client, _acl_id, _) = setup(&env);

        let caller = Address::generate(&env);
        env.mock_all_auths();

        let result = client.try_protected_function(&caller);
        assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
    }
}
