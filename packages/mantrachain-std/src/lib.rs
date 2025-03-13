#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

mod serde;
pub mod shim;

#[allow(deprecated, unused_imports, clippy::large_enum_variant)]
pub mod types;

pub use shim::{cosmwasm_to_proto_coins, try_proto_to_cosmwasm_coins};
