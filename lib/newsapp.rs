#[cfg(any(
    all(feature = "net_async", not(feature = "net_block")),
    all(feature = "net_block", not(feature = "net_async"))
))]
pub mod api;

#[cfg(any(
    all(feature = "net_async", not(feature = "net_block")),
    all(feature = "net_block", not(feature = "net_async"))
))]
pub use api::*;

mod article;

#[cfg(feature = "cache")]
pub mod cache;

pub use article::*;

pub use color_eyre::eyre::*;
pub use color_eyre::install as color_eyre_install;
