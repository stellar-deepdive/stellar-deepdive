use soroban_sdk::{contracttype, symbol_short, Address, Env, String, Symbol};

// ============================================================================
// Event Topics
// ============================================================================

pub const ENTRY_REGISTERED: Symbol = symbol_short!("REG_ADD");
pub const ENTRY_UPDATED: Symbol = symbol_short!("REG_UPD");
pub const ENTRY_REMOVED: Symbol = symbol_short!("REG_REM");
pub const REGISTRY_TOPIC: Symbol = symbol_short!("REGISTRY");

// ============================================================================
// Event Structures
// ============================================================================

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryRegistered {
    pub key: String,
    pub owner: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryUpdated {
    pub key: String,
    pub owner: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryRemoved {
    pub key: String,
}

// ============================================================================
// Emit Helpers
// ============================================================================

pub fn emit_registered(env: &Env, key: String, owner: Address) {
    env.events()
        .publish((ENTRY_REGISTERED, REGISTRY_TOPIC), EntryRegistered { key, owner });
}

pub fn emit_updated(env: &Env, key: String, owner: Address) {
    env.events()
        .publish((ENTRY_UPDATED, REGISTRY_TOPIC), EntryUpdated { key, owner });
}

pub fn emit_removed(env: &Env, key: String) {
    env.events()
        .publish((ENTRY_REMOVED, REGISTRY_TOPIC), EntryRemoved { key });
}
