//! # amethyst_assets
//!
//! Asset management crate.
//! Designed with the following goals in mind:
//!
//! * Extensibility
//! * Asynchronous & Parallel using Rayon
//! * Allow different sources

#![doc(
    html_logo_url = "https://amethyst.rs/brand/logo-standard.svg",
    html_root_url = "https://docs.amethyst.rs/stable"
)]
#![warn(missing_docs, rust_2018_idioms, rust_2018_compatibility)]

pub use rayon::ThreadPool;

mod asset;
mod bundle;
mod cache;
pub mod error;
mod formats;
mod loader;
pub mod prefab;
mod processor;
mod progress;
mod simple_importer;
mod source;
mod storage;

pub use atelier_assets::loader::{
    handle::{AssetHandle, GenericHandle, Handle, WeakHandle},
    storage::LoadHandle,
};
// used in macros. Private API otherwise.
#[doc(hidden)]
pub use {erased_serde, inventory, lazy_static};

#[doc(hidden)]
#[cfg(feature = "json")]
pub use crate::formats::JsonFormat;
pub use crate::{
    asset::{Asset, Format, FormatValue, ProcessableAsset, SerializableFormat},
    bundle::{start_asset_daemon, LoaderBundle},
    cache::Cache,
    formats::RonFormat,
    loader::{create_asset_type, AssetUuid, DefaultLoader, LoadStatus, Loader},
    processor::{AssetProcessorSystem, ProcessingQueue, ProcessingState},
    progress::{Completion, Progress, ProgressCounter, Tracker},
    simple_importer::{SimpleImporter, SourceFileImporter},
    source::{Directory, Source},
    storage::AssetStorage,
};
