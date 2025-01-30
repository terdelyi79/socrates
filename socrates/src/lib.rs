//! A persistence module optimized for:
//!  - Fast and easy development
//!  - Unit testability
//!  - Team work using source control
//!  - Easy upgrade
//!  - Zero dependencies
//! 
//! Implementation is based on the CQRS (Command and Query Responsibility Segregation) and event sourcing patterns.

pub mod error;
pub mod event;
pub mod file_system_storage;
pub mod sink;
pub mod storage;
pub  mod json;