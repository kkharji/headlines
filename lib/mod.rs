#![feature(derive_default_enum, default_free_fn)]

#[cfg(any(feature = "net_block", feature = "net_async"))]
pub mod client;
mod data;
mod util;

pub use data::article::{Article, Articles};
pub use data::*;
// garticle::{category, lang, scope, Article, Articles};
pub use util::external::*;
pub use util::HLError;
