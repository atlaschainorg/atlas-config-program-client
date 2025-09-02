#![allow(clippy::arithmetic_side_effects)]

mod generated;
mod hooked;
#[deprecated(since = "1.0.0", note = "use `atlas_config_interface` crate instead")]
#[cfg(feature = "serde")]
pub mod instructions_bincode;

pub use {
    generated::{programs::ATLAS_CONFIG_ID as ID, *},
    hooked::*,
};
