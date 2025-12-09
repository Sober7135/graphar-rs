# GraphAr-RS: Safe Rust Bindings for Apache GraphAr (Incubating)

GraphAr-RS exposes the Apache GraphAr C++ library to Rust through the `cxx` bridge. It lets you read and write GraphAr graphs, work with Arrow-backed property columns, and stay within Rust's safety guarantees when manipulating graph metadata and data files.

## What Is Apache GraphAr?

- **Purpose:** GraphAr defines a storage format and C++ library for large-scale property graphs stored on disk.
- **Metadata-first design:** YAML metadata (`GraphInfo`, `VertexInfo`, `EdgeInfo`) describes labels, schemas, chunk sizes, and file layouts.
- **Property groups:** Properties are batched into columnar files (CSV/Parquet/ORC) to balance I/O and schema evolution.
- **Adjacency layouts:** Multiple adjacency lists (ordered/unordered by source or destination) support different traversal patterns.
- **Chunking:** Vertices and edges are partitioned into chunks to enable parallel loading and bounded-memory processing.
- **Arrow integration:** Arrow memory layouts and kernels underpin property storage and many higher-level algorithms.

Typical C++ usage constructs metadata, dumps vertex/edge chunks with builders, and reads them via iterators with optional label or property filters. GraphAr-RS mirrors that workflow in Rust.

## Crate Layout

- `src/ffi.rs` – `cxx::bridge` declarations and minimal wrappers around the GraphAr C++ surface exposed through `include/graphar_rs.h`.
- `include/graphar_rs.h` / `src/graphar_rs.cc` – C++ shim that normalizes GraphAr's API (fixed-width integers, smart pointers, error translation) for the FFI boundary.
- `src/graph_info.rs` – Safe Rust wrappers for metadata (`GraphInfo`, `VertexInfo`, `EdgeInfo`, `Property`, `PropertyGroup`, `DataType`, `InfoVersion`, `AdjacentList`, ...).
- `src/graph_builder.rs` – Safe vertex/edge builders with `add_property<T>` helpers and `dump()` to persist data chunks.
- `src/graph_reader.rs` – Readers and iterators (`Vertices`, `Edges`, `VertexIter`, `EdgeIter`) with typed property accessors.
- `build.rs` – Invokes CMake to build the vendored GraphAr sources, then compiles and links the shim via `cxx_build`.
- Submodules – `incubator-graphar/` (upstream C++ sources) and `incubator-graphar-testing/` (fixtures used by examples/tests).

## Getting Started

### Prerequisites

Install the Rust toolchain, CMake, and a C++17 compiler. Ensure Arrow development headers and libraries required by GraphAr are resolvable during the build.

### Fetch Submodules

```bash
git submodule update --init --recursive
```

### Build

```bash
cargo build            # add --release for optimized builds
```

### Run Tests

```bash
cargo test
```

## Examples

Two examples demonstrate end-to-end usage:

- `read` – Loads `incubator-graphar-testing/modern_graph/modern_graph.graph.yml`, iterates vertices and edges, and prints identifiers and properties.

  ```bash
  cargo run --example read
  ```

- `dump` – Builds metadata and dumps a toy dataset into `/tmp/test_graphar/` using the builder APIs.

  ```bash
  cargo run --example dump
  ```

See `examples/read.rs` and `examples/dump.rs` for the full flows.

## Type Mapping

| `graphar` (C++) | `graphar-rs` (Rust) |
| --- | --- |
| `graphar::GraphInfo` | `graph_info::GraphInfo` |
| `graphar::InfoVersion` / `graphar::ConstInfoVersion` | `graph_info::InfoVersion` |
| `graphar::DataType` | `graph_info::DataType` |
| `graphar::Property` | `graph_info::Property` |
| `graphar::PropertyGroup` | `graph_info::PropertyGroup` |
| `graphar::PropertyGroupVector` | `graph_info::PropertyGroupVector` |
| `graphar::VertexInfo` | `graph_info::VertexInfo` |
| `graphar::EdgeInfo` | `graph_info::EdgeInfo` |
| `graphar::AdjacentList` | `graph_info::AdjacentList` |
| `graphar::AdjacentListVector` | `graph_info::AdjacentListVector` |
| `graphar::FileType` | `graph_info::FileType` |
| `graphar::Type` | `graph_info::Type` |
| `graphar::Cardinality` | `graph_info::Cardinality` |
| `graphar::AdjListType` | `graph_info::AdjListType` |
| `graphar::VerticesCollection` | `graph_reader::Vertices` |
| `graphar::VertexIter` | `graph_reader::VertexIter` |
| `graphar::EdgesCollection` | `graph_reader::Edges` |
| `graphar::EdgeIter` | `graph_reader::EdgeIter` |
| `graphar::Vertex` | not exposed directly (use `VertexIter`/property fns) |
| `graphar::Edge` | not exposed directly (use `EdgeIter`/property fns) |
| `graphar::Expression` | not exposed directly (use `expression_*` helpers) |
| `graphar::builder::Vertex` | `graph_builder::Vertex` |
| `graphar::builder::VerticesBuilder` | `graph_builder::VerticesBuilder` |
| `graphar::builder::Edge` | `graph_builder::Edge` |
| `graphar::builder::EdgesBuilder` | `graph_builder::EdgesBuilder` |
| `std::vector<graphar::Property>` | `graph_info::PropertyVec` |

## Current Limitations

- List-, date-, and timestamp-typed properties are not yet supported by the safe builders or iterators.
- Some GraphAr APIs remain unexposed.
- Can only be compiled in Dockerfile.

## License

GraphAr-RS inherits the Apache License 2.0 from the upstream project. See [LICENSE](LICENSE).
