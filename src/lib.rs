//! The account service's public profile contract and authenticated SDK client.
pub mod api;

pub use api::server::{me_path, me_response_json};
