mod capabilities;
mod client;
mod error;

pub use client::Client;
pub use error::Error;

use capabilities::capabilities;

pub type Result<T, E = Error> = std::result::Result<T, E>;
