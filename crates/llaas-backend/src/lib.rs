//! Backend service runtime for LLAAS.
//!
//! This crate will own Actix server wiring, GraphQL and REST routes, frontend
//! static serving, Apalis board mounting, and worker startup.

pub mod api;
pub mod client;
