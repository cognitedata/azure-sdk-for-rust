#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-07-01-preview")]
pub mod package_2023_07_01_preview;
#[cfg(all(feature = "package-2023-07-01-preview", not(feature = "no-default-tag")))]
pub use package_2023_07_01_preview::*;
