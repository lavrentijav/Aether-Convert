# Aether-Convert

**Standalone Anvil → Aether KV world migration tool.**

Aether-Convert reads a legacy Minecraft **Anvil** world (`region/*.mca`) and
writes it into the [Aether Engine](https://github.com/lavrentijav/Aether-Engine)
KV world store: palette-compressed, Morton-indexed sub-chunks serialized as
Zstandard-compressed `ASC1` blobs in a Fjall key/value database.

This repository is a **self-contained clone** of the engine's `aether-convert`
crate. It vendors the minimal slice of the engine's world model (`src/world/`)
so it builds and ships on its own, while staying **byte-compatible** with what
the engine reads back.

## Build

```bash
cargo build --release
cargo test
```

## Usage

```bash
# world folder (containing region/) or a region/ directory  ->  KV store dir
aether-convert <world-dir> <output-dir> [--threads N] [--mem]
```

| Flag | Meaning |
|------|---------|
| `--threads N` | Worker threads (default: available CPU parallelism). |
| `--mem` | Dry run into an in-memory store (nothing persisted). |

Example:

```bash
cargo run --release -- ~/.minecraft/saves/MyWorld ./my-world-kv
cargo run --release -- ~/.minecraft/saves/MyWorld --mem   # validate only
```

The tool prints an audit report — regions, chunks and sub-chunks processed, a
run checksum, throughput, and any non-fatal per-chunk errors.

## What it handles

- Anvil region framing (8 KiB header, sectorised chunk payloads).
- zlib / uncompressed chunk compression.
- The 1.16+ `block_states` palette + packed `data` layout (YZX order).
- Deterministic block-name → dense engine-id mapping with property inference
  (solid / collision / redstone), so no block is dropped.
- Parallel per-region conversion with an order-independent audit checksum.

## Not yet (Phase 4 hardening, tracked in the engine roadmap)

- gzip / LZ4 chunk compression.
- Full block-state property mapping from block properties.
- Resumable / incremental runs.

## License

MIT OR Apache-2.0. See [LICENSE](LICENSE).
