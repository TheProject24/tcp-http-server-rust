//! This crate provides the underlying data structures and parsing utilities
//! for a custom HTTP server built over TCP. It exports modules for
//! parsing HTTP bodies, managing headers, constructing requests, and building responses.

pub mod body;
pub mod headers;
pub mod request;
pub mod response;
