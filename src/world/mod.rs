//! Inlined world model & storage format.
//!
//! Aether-Convert is a standalone clone of the Aether Engine's `aether-convert`
//! crate. So it can build without the engine workspace, it vendors the minimal
//! slice of `aether-world` it needs to write the engine's on-disk format:
//! the SoA [`SubChunk`] model, [`Palette`] compression, Morton indexing and the
//! KV + Zstandard [`storage`] layer.
//!
//! This layout is kept **byte-compatible** with the engine's `aether-world`
//! blob format (magic `ASC1`) and key encoding, so a converted world loads
//! directly in the engine.

pub mod block;
pub mod morton;
pub mod palette;
pub mod storage;
pub mod subchunk;

pub use block::{BlockProperties, BlockStateId};
pub use palette::{PackedArray, Palette};
pub use storage::format::{FormatError, SubChunkKey};
pub use storage::{KvBackend, MemStore, StorageError, WorldStorage};
pub use subchunk::{Mask, SubChunk, DIM, MASK_WORDS, VOLUME};

#[cfg(feature = "fjall")]
pub use storage::FjallStore;
