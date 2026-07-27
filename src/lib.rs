//! # Aether-Convert (standalone)
//!
//! A self-contained clone of the Aether Engine's `aether-convert` crate that
//! migrates a legacy **Anvil** world (`region/*.mca`) into the Aether KV world
//! store — without depending on the engine workspace.
//!
//! It vendors the minimal slice of the engine's world model it needs (see
//! [`world`]) so the output is **byte-compatible** with what the engine reads.
//!
//! Pipeline:
//! 1. [`anvil`] reads region files and decompresses each chunk's NBT.
//! 2. [`nbt`] parses the chunk tree.
//! 3. [`block_map`] maps Anvil block names to dense engine ids + properties.
//! 4. [`convert`] rebuilds [`world::SubChunk`]s and writes them to a
//!    [`world::WorldStorage`], in parallel across region files, with an audit
//!    [`convert::ConversionReport`].
//!
//! Use it as a library or through the bundled `aether-convert` CLI.

pub mod anvil;
pub mod block_map;
pub mod convert;
pub mod nbt;
pub mod world;

pub use block_map::{BlockRegistry, Interner};
pub use convert::{convert_world, ConversionReport};
