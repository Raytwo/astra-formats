mod asset;

mod book;
mod bundle;
mod msbt;
mod sprite_atlas;

pub mod texture;

pub use anyhow as error;
pub use binrw;
pub use image;
pub use indexmap;

pub use asset::*;

pub use book::*;
pub use bundle::*;
pub use msbt::MessageMap;
pub use sprite_atlas::SpriteAtlasWrapper;

#[cfg(feature = "ffi")]
mod ffi;

#[cfg(feature = "msbt_script")]
mod astra_script;

#[cfg(feature = "msbt_script")]
mod msbt_script;

#[cfg(feature = "msbt_script")]
pub use astra_script::{
    pack_astra_script, parse_astra_script, parse_astra_script_entry, ParseError,
};

#[cfg(feature = "msbt_script")]
pub use msbt_script::{
    pack_msbt_entries, pack_msbt_entry, parse_msbt_entry, parse_msbt_script, MsbtToken,
};
