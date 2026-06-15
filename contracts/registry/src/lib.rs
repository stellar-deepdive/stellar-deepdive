#![no_std]

mod errors;
mod events;

use errors::Error;
use events::{emit_registered, emit_removed, emit_updated};
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// ============================================================================
// Constants
// ============================================================================

/// Maximum byte length for a registry key
const MAX_KEY_LEN: u32 = 64;

/// Maximum byte length for a registry value
const MAX_VALUE_LEN: u32 = 512;

// ============================================================================
// Data Types
// ============================================================================

/// A single registry entry owned by an address.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    pub owner: Address,
    pub value: String,
    pub created_at: u64,
    pub updated_at: u64,
}

// ============================================================================
// Storage Keys
// ============================================================================

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Singleton admin address (instance storage)
    Admin,
    /// Total number of registered entries (instance storage)
    EntryCount,
    /// Individual entry keyed by its string key (persistent storage)
    Entry(String),
}

// ============================================================================
// Contract
// ============================================================================

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    // ========================================================================
    // Lifecycle
    // ========================================================================

    /// Initialize the registry with an admin address.
    /// Can only be called once.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::EntryCount, &0u64);
        Ok(())
    }

    // ========================================================================
    // Write Operations
    // ========================================================================

    /// Register a new key → value pair. The caller becomes the entry owner.
    /// Fails if the key already exists.
    pub fn register(env: Env, caller: Address, key: String, value: String) -> Result<(), Error> {
        Self::assert_initialized(&env)?;
        caller.require_auth();

        if key.len() == 0 || key.len() > MAX_KEY_LEN {
            return Err(Error::InvalidInput);
        }
        if value.len() > MAX_VALUE_LEN {
            return Err(Error::InvalidInput);
        }
        if env.storage().persistent().has(&DataKey::Entry(key.clone())) {
            return Err(Error::AlreadyExists);
        }

        let now = env.ledger().timestamp();
        let entry = Entry {
            owner: caller.clone(),
            value,
            created_at: now,
            updated_at: now,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Entry(key.clone()), &entry);

        let mut count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::EntryCount)
            .unwrap_or(0);
        count += 1;
        env.storage().instance().set(&DataKey::EntryCount, &count);

        emit_registered(&env, key, caller);
        Ok(())
    }

    /// Update the value for an existing entry. Only the entry owner may update.
    pub fn update(env: Env, caller: Address, key: String, new_value: String) -> Result<(), Error> {
        Self::assert_initialized(&env)?;
        caller.require_auth();

        if new_value.len() > MAX_VALUE_LEN {
            return Err(Error::InvalidInput);
        }

        let mut entry: Entry = env
            .storage()
            .persistent()
            .get(&DataKey::Entry(key.clone()))
            .ok_or(Error::NotFound)?;

        if entry.owner != caller {
            return Err(Error::UnauthorizedOwner);
        }

        entry.value = new_value;
        entry.updated_at = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&DataKey::Entry(key.clone()), &entry);

        emit_updated(&env, key, caller);
        Ok(())
    }

    /// Remove an entry. The entry owner or the admin may remove.
    pub fn remove(env: Env, caller: Address, key: String) -> Result<(), Error> {
        Self::assert_initialized(&env)?;
        caller.require_auth();

        let entry: Entry = env
            .storage()
            .persistent()
            .get(&DataKey::Entry(key.clone()))
            .ok_or(Error::NotFound)?;

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;

        if caller != entry.owner && caller != admin {
            return Err(Error::UnauthorizedOwner);
        }

        env.storage()
            .persistent()
            .remove(&DataKey::Entry(key.clone()));

        let count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::EntryCount)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::EntryCount, &count.saturating_sub(1));

        emit_removed(&env, key);
        Ok(())
    }

    /// Transfer ownership of an entry to a new owner.
    /// Only the current entry owner may transfer.
    pub fn transfer_ownership(
        env: Env,
        caller: Address,
        key: String,
        new_owner: Address,
    ) -> Result<(), Error> {
        Self::assert_initialized(&env)?;
        caller.require_auth();

        let mut entry: Entry = env
            .storage()
            .persistent()
            .get(&DataKey::Entry(key.clone()))
            .ok_or(Error::NotFound)?;

        if entry.owner != caller {
            return Err(Error::UnauthorizedOwner);
        }

        entry.owner = new_owner;
        entry.updated_at = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&DataKey::Entry(key), &entry);

        Ok(())
    }

    // ========================================================================
    // Admin Operations
    // ========================================================================

    /// Transfer the admin role to a new address. Only the current admin may call.
    pub fn transfer_admin(env: Env, caller: Address, new_admin: Address) -> Result<(), Error> {
        Self::assert_initialized(&env)?;
        caller.require_auth();

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;

        if caller != admin {
            return Err(Error::UnauthorizedAdmin);
        }

        env.storage().instance().set(&DataKey::Admin, &new_admin);
        Ok(())
    }

    // ========================================================================
    // Query Operations
    // ========================================================================

    /// Look up a registry entry by key.
    pub fn get(env: Env, key: String) -> Result<Entry, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Entry(key))
            .ok_or(Error::NotFound)
    }

    /// Check whether a key is currently registered.
    pub fn exists(env: Env, key: String) -> bool {
        env.storage().persistent().has(&DataKey::Entry(key))
    }

    /// Total number of registered entries.
    pub fn entry_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::EntryCount)
            .unwrap_or(0)
    }

    /// Return the admin address.
    pub fn admin(env: Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    // ========================================================================
    // Internal Helpers
    // ========================================================================

    fn assert_initialized(env: &Env) -> Result<(), Error> {
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NotInitialized);
        }
        Ok(())
    }
}

mod test;
