//! Copyright (c) 2022 MASSA LABS <info@massa.net>
//! All the structures that are used everywhere
//!
#![warn(missing_docs)]
#![warn(unused_crate_dependencies)]
#![feature(bound_map)]
#![feature(int_roundings)]
#![feature(iter_intersperse)]

use massa_time::MassaTime;
use serde::{Deserialize, Serialize};

/// address related structures
pub mod address;
/// block-related structures
pub mod block;
/// node configuration
pub mod config;
/// datastore serialization / deserialization
pub mod datastore;
/// endorsements
pub mod endorsement;
/// models error
pub mod error;
/// execution
pub mod execution;
/// ledger structures
pub mod ledger;
/// node related structure
pub mod node;
/// operations
pub mod operation;
/// page
pub mod page;
/// rolls
pub mod rolls;
/// slots
pub mod slot;

/// Dumb utils function to display nicely boolean value
fn display_if_true(value: bool, text: &str) -> String {
    if value {
        format!("[{}]", text)
    } else {
        String::from("")
    }
}

/// Just a wrapper with a optional beginning and end
#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
pub struct TimeInterval {
    /// optional start slot
    pub start: Option<MassaTime>,
    /// optional end slot
    pub end: Option<MassaTime>,
}

/// SCRUD operations
#[derive(strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum ScrudOperation {
    /// search operation
    Search,
    /// create operation
    Create,
    /// read operation
    Read,
    /// update operation
    Update,
    /// delete operation
    Delete,
}

/// Bootsrap lists types
#[derive(strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum ListType {
    /// contains banned entry
    Blacklist,
    /// contains allowed entry
    Whitelist,
}
