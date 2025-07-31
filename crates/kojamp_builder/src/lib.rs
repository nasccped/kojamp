//! Build library designed to work with the `build.rs` file during kojamp app
//! compilation.
//!
//! It dismisses manually toml file reading/unwrapping by using it's own
//! analysis code implementation with toml crate!
//!
//! ## Tools
//!
//! It provides two usefull tools:
//!
//! 1. [`Config`][config::Config] from [`config`] module
//! 2. [`ErrorKind`][config::ErrorKind] from [`config`] module also
//!
//! [`Config`][config::Config] is a struct that provides essentials app data
//! fields, (name, author, version, ...) and [`ErrorKind`][config::ErrorKind]
//! provides error variants when toml reading fails.

pub mod config;
