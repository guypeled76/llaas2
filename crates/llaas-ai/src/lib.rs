//! AI and model integrations for LLAAS.
//!
//! Heavy model dependencies such as rust-bert, tch, and TTS backends belong
//! here so they do not leak into clients or pure domain crates.