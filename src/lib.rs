#![deny(bare_trait_objects)]

mod signer;
mod credential;
mod error;

pub use signer::Signer;
pub use credential::Credential;
pub use error::Error;

#[cfg(feature="reqwest")]
mod client;
#[cfg(feature="reqwest")]
pub use client::Client;
