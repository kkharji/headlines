#[cfg(any(
    all(feature = "net_async", not(feature = "net_block")),
    all(feature = "net_block", not(feature = "net_async"))
))]
mod api;

mod article;

#[cfg(any(
    all(feature = "net_async", not(feature = "net_block")),
    all(feature = "net_block", not(feature = "net_async"))
))]
pub use api::{endpoint::*, error::*, NewsApi};

#[cfg(feature = "cache")]
pub mod cache;

pub use article::*;
pub use eyre::*;
