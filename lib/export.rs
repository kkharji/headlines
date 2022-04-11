#[cfg(any(feature = "net_block", feature = "net_async"))]
mod api;

mod article;

#[cfg(any(feature = "net_block", feature = "net_async"))]
pub use api::{endpoint::*, error::*, NewsApi};

#[cfg(feature = "cache")]
pub mod cache;

pub use article::*;
pub use eyre::*;
pub use strum;
