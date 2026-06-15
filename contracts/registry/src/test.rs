#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

// ============================================================================
// Test Helpers
// ============================================================================

fn setup() -> (Env, RegistryContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, RegistryContract);
    let client = RegistryContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    (env, client, admin)
}

fn key(env: &Env, s: &str) -> String {
    String::from_str(env, s)
}

fn val(env: &Env, s: &str) -> String {
    String::from_str(env, s)
}

// ============================================================================
// Initialization Tests
// ============================================================================

#[test]
fn test_initialize() {
    let (env, client, admin) = setup();
    assert_eq!(client.admin(), admin);
    assert_eq!(client.entry_count(), 0);
    let _ = env;
}

#[test]
fn test_double_initialize_fails() {
    let (env, client, admin) = setup();
    let result = client.try_initialize(&admin);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
    let _ = env;
}

// ============================================================================
// Register Tests
// ============================================================================

#[test]
fn test_register_success() {
    let (env, client, _admin) = setup();

    let owner = Address::generate(&env);
    client.register(&owner, &key(&env, "my.key"), &val(&env, "hello"));

    assert!(client.exists(&key(&env, "my.key")));
    assert_eq!(client.entry_count(), 1);

    let entry = client.get(&key(&env, "my.key"));
    assert_eq!(entry.owner, owner);
    assert_eq!(entry.value, val(&env, "hello"));
    assert_eq!(entry.created_at, entry.updated_at);
}

#[test]
fn test_register_duplicate_fails() {
    let (env, client, _admin) = setup();

    let owner = Address::generate(&env);
    client.register(&owner, &key(&env, "dup"), &val(&env, "first"));

    let result = client.try_register(&owner, &key(&env, "dup"), &val(&env, "second"));
    assert_eq!(result, Err(Ok(Error::AlreadyExists)));
}

#[test]
fn test_register_empty_key_fails() {
    let (env, client, _admin) = setup();
    let owner = Address::generate(&env);

    let result = client.try_register(&owner, &key(&env, ""), &val(&env, "value"));
    assert_eq!(result, Err(Ok(Error::InvalidInput)));
}

// ============================================================================
// Update Tests
// ============================================================================

#[test]
fn test_update_success() {
    let (env, client, _admin) = setup();

    let owner = Address::generate(&env);
    client.register(&owner, &key(&env, "k"), &val(&env, "v1"));

    env.ledger().with_mut(|li| li.timestamp = 100);
    client.update(&owner, &key(&env, "k"), &val(&env, "v2"));

    let entry = client.get(&key(&env, "k"));
    assert_eq!(entry.value, val(&env, "v2"));
    assert!(entry.updated_at > entry.created_at);
}

#[test]
fn test_update_wrong_owner_fails() {
    let (env, client, _admin) = setup();

    let owner = Address::generate(&env);
    let other = Address::generate(&env);
    client.register(&owner, &key(&env, "k"), &val(&env, "v1"));

    let result = client.try_update(&other, &key(&env, "k"), &val(&env, "v2"));
    assert_eq!(result, Err(Ok(Error::UnauthorizedOwner)));
}

#[test]
fn test_update_nonexistent_fails() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);

    let result = client.try_update(&caller, &key(&env, "ghost"), &val(&env, "v"));
    assert_eq!(result, Err(Ok(Error::NotFound)));
}

// ============================================================================
// Remove Tests
// ============================================================================

#[test]
fn test_remove_by_owner() {
    let (env, client, _admin) = setup();

    let owner = Address::generate(&env);
    client.register(&owner, &key(&env, "k"), &val(&env, "v"));
    assert_eq!(client.entry_count(), 1);

    client.remove(&owner, &key(&env, "k"));
    assert!(!client.exists(&key(&env, "k")));
    assert_eq!(client.entry_count(), 0);
}

#[test]
fn test_remove_by_admin() {
    let (env, client, admin) = setup();

    let owner = Address::generate(&env);
    client.register(&owner, &key(&env, "k"), &val(&env, "v"));

    client.remove(&admin, &key(&env, "k"));
    assert!(!client.exists(&key(&env, "k")));
}

#[test]
fn test_remove_unauthorized_fails() {
    let (env, client, _admin) = setup();

    let owner = Address::generate(&env);
    let stranger = Address::generate(&env);
    client.register(&owner, &key(&env, "k"), &val(&env, "v"));

    let result = client.try_remove(&stranger, &key(&env, "k"));
    assert_eq!(result, Err(Ok(Error::UnauthorizedOwner)));
}

// ============================================================================
// Transfer Ownership Tests
// ============================================================================

#[test]
fn test_transfer_ownership() {
    let (env, client, _admin) = setup();

    let owner = Address::generate(&env);
    let new_owner = Address::generate(&env);
    client.register(&owner, &key(&env, "k"), &val(&env, "v"));

    client.transfer_ownership(&owner, &key(&env, "k"), &new_owner);

    let entry = client.get(&key(&env, "k"));
    assert_eq!(entry.owner, new_owner);

    // Old owner can no longer update
    let result = client.try_update(&owner, &key(&env, "k"), &val(&env, "new"));
    assert_eq!(result, Err(Ok(Error::UnauthorizedOwner)));
}

// ============================================================================
// Admin Transfer Tests
// ============================================================================

#[test]
fn test_transfer_admin() {
    let (env, client, admin) = setup();
    let new_admin = Address::generate(&env);

    client.transfer_admin(&admin, &new_admin);
    assert_eq!(client.admin(), new_admin);
}

#[test]
fn test_transfer_admin_unauthorized_fails() {
    let (env, client, _admin) = setup();
    let stranger = Address::generate(&env);
    let new_admin = Address::generate(&env);

    let result = client.try_transfer_admin(&stranger, &new_admin);
    assert_eq!(result, Err(Ok(Error::UnauthorizedAdmin)));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_exists_returns_false_for_unknown_key() {
    let (env, client, _admin) = setup();
    assert!(!client.exists(&key(&env, "nope")));
}

#[test]
fn test_entry_count_increments_and_decrements() {
    let (env, client, _admin) = setup();

    let owner = Address::generate(&env);
    client.register(&owner, &key(&env, "a"), &val(&env, "1"));
    client.register(&owner, &key(&env, "b"), &val(&env, "2"));
    assert_eq!(client.entry_count(), 2);

    client.remove(&owner, &key(&env, "a"));
    assert_eq!(client.entry_count(), 1);
}
