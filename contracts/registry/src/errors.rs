use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Contract has not been initialized
    NotInitialized = 1,
    /// Contract has already been initialized
    AlreadyInitialized = 2,
    /// Caller is not the admin
    UnauthorizedAdmin = 3,
    /// Caller is not the entry owner
    UnauthorizedOwner = 4,
    /// An entry with this key already exists
    AlreadyExists = 5,
    /// No entry found for the given key
    NotFound = 6,
    /// Key or value exceeds allowed length
    InvalidInput = 7,
}
