pub mod commands;
pub mod config;
pub mod error;
pub mod read;
pub mod stats;
pub mod transform;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;
