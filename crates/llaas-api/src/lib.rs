//! Public API contract for LLAAS.
//!
//! This crate will own code-first GraphQL schema-facing types, public REST DTOs,
//! SDL export support, operation documents, and conversions to and from
//! `llaas-core` domain types.

pub mod graphql;

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

pub mod rest;
