# HGraphs development guide

This file applies to the entire workspace. HGraphs is intended to be a
high-performance, generic, reliable Rust hypergraph library with efficient
`extern "C"` bridges for Julia and Python.

## Design priorities

Use this order when requirements compete:

1. Correctness, memory safety, and explicit failure handling.
2. Predictable performance and memory use.
3. Generic, composable graph concepts and algorithms.
4. Portability, including `no_std` where practical and C-callable bindings.
5. A small, carefully justified dependency surface.

Do not trade correctness or defined behavior for benchmark improvements.
Measure performance-sensitive changes instead of relying on intuition.

## Workspace architecture

- `hgraphs-core` defines lightweight graph concepts, index types, property-map
  concepts, and shared errors. Keep it `#![no_std]` compatible; use `core` and,
  only when allocation-facing APIs require it, `alloc`.
- `hgraphs` contains concrete storage, algorithms, generators, projections,
  and optional I/O. Algorithms should depend on capabilities from
  `hgraphs-core`, not on a particular storage implementation.
- Binding crates are thin adapters. Domain logic belongs in the Rust library,
  not in Python, Julia, C, or macro glue.
- Keep exported `extern "C"` functions in a dedicated FFI crate or clearly
  isolated module. Do not weaken or de-genericize the native Rust API merely to
  make it directly FFI-safe.

Avoid dependency cycles and avoid making the core crate aware of a concrete
container, runtime, serialization format, or language binding.

## Generic API design

Follow the spirit of the Boost Graph Library: model small capabilities and
write algorithms against the weakest sufficient set of capabilities.

- Prefer focused traits such as node iteration, edge incidence, mutability,
  directedness, and property-map access over one monolithic graph trait.
- Keep node and hyperedge identifiers as distinct strong types. Do not assume
  identifiers are `usize`, contiguous, stable after mutation, or interchangeable.
- Prefer iterators, slices, and generic static dispatch in hot Rust APIs. Avoid
  mandatory `Box`, trait objects, reference counting, and callback indirection.
- Offer bulk operations when they materially reduce validation, allocation, or
  FFI overhead.
- Make directedness and other compile-time graph properties explicit in types
  or traits when doing so enables correctness checks or optimization.
- Keep algorithms separate from storage. New algorithms should work with any
  graph satisfying their documented trait bounds.
- Make ordinary graph construction and mutation generic over the weakest
  sufficient capability traits. Do not duplicate operations such as adding
  nodes, adding hyperedges, or adding incidences for every storage backend when
  one generic implementation can express them.
- Make graph generators generic over the construction and mutation capabilities
  they require. A generator must not select or depend on a concrete storage
  layout; callers choose the output storage type.
- Put genuinely layout-dependent constructors, import paths, capacity controls,
  and tuning operations on the concrete storage type as associated functions.
  Name and document them by the representation guarantee they provide; do not
  add layout-specific concerns to common graph traits.
- Generic construction and generators may use a small factory or builder trait
  when `Default` plus mutation traits cannot express fallible initialization.

Every public operation must document identifier validity, mutation and
invalidation rules, ordering guarantees, allocation behavior, and asymptotic
time and space complexity when these are not obvious.

## Errors, absence, and panic freedom

Safe public library APIs must not panic for any input or recoverable failure.

- Use `Option<T>` only for ordinary absence that requires no explanation.
- Use `Result<T, E>` when an operation can fail. Use `Result<Option<T>, E>` when
  absence and failure are both meaningful.
- Prefer small, typed, non-string error enums that callers can exhaustively
  handle. Preserve the underlying cause when it is useful.
- Mutating operations should either succeed completely or return an error
  without leaving a partially updated or internally inconsistent graph.
- Treat arithmetic overflow, invalid IDs, invalid state, capacity exhaustion,
  malformed input, and unsupported operations as explicit error paths.
- Use fallible reservation before growth and propagate `TryReserveError` or a
  meaningful library error. Do not hide potentially large allocations.
- Do not use `unwrap`, `expect`, indexing, `panic!`, `assert!`, `todo!`,
  `unimplemented!`, or `unreachable!` on reachable library paths. Assertions
  are appropriate in tests. Prefer checked access and explicit control flow.
- Do not use a panic as an internal error transport mechanism.

Rust's global allocator can abort the process on unrecoverable out-of-memory
conditions. Within that platform limitation, make allocation failures fallible
where the standard APIs permit and clearly document any operation that cannot
provide that guarantee.

Unchecked APIs may exist only when they provide demonstrated value. They must
be `unsafe`, name the unchecked condition, document every caller obligation in
a `# Safety` section, and have a checked safe counterpart. Each unsafe block
must include a local `SAFETY:` explanation. Keep unsafe code small and out of
generic algorithms whenever possible.

## Performance and allocation

- Favor cache-friendly layouts, compact indices, contiguous storage, and
  iteration patterns over pointer-heavy object graphs where measurements agree.
- Avoid cloning, collecting, hashing, and allocating in inner loops unless the
  algorithm requires them.
- Reuse caller-provided workspaces for algorithms that otherwise allocate on
  every call. Also provide an ergonomic convenience API when appropriate.
- Preserve deterministic behavior where practical. If iteration order is
  intentionally unspecified, document it and never make correctness depend on it.
- Add or update benchmarks for changes to storage layouts, traversal kernels,
  bulk operations, or FFI call patterns. Report meaningful before/after data for
  performance claims.
- An optimization must not silently change error behavior, overflow behavior,
  ordering guarantees, or graph invariants.

## Dependencies and portability

- Prefer `core`, `alloc`, and the standard library over adding a dependency.
- Add a dependency only when it provides substantial, maintained functionality
  that would be risky or costly to reproduce. Explain the need in the change.
- Disable dependency default features when they are unnecessary. Audit feature
  flags, transitive dependencies, licenses, MSRV, `no_std` support, and binary
  impact.
- Do not add convenience-only error, iterator, or derive crates for behavior
  that is straightforward to implement locally.
- Preserve the workspace MSRV declared by the crates, currently Rust 1.85.1,
  unless the project explicitly decides to raise it.
- Gate I/O, parallelism, serialization, and language runtimes behind optional
  features or separate crates rather than pulling them into the core.

## C-callable language bindings

The native generic Rust API and the C-callable binding surface are separate
layers. The exported functions are an implementation bridge for Julia and
Python, not a promise to maintain a stable public C API. They may evolve in
lockstep with the bindings.

- Expose opaque handles plus explicit constructor, destructor, query, and
  mutation functions. Make ownership and borrowing rules unambiguous.
- Only expose FFI-safe representations: fixed-width integers when appropriate,
  `#[repr(C)]` data structures when layouts must cross the boundary, explicit
  tagged option/result representations, and pointer/length pairs.
- Export concrete entry points for the graph types and operations required by
  each binding. Keep the implementation generic behind those entry points;
  Rust traits and type parameters do not cross the C boundary.
- Never expose Rust references, slices, `Vec`, `String`, enums without an
  explicit representation, trait objects, generics, or unwinding across C.
- Return status/error codes and write values through validated out-parameters.
  Provide a defined way to retrieve structured error details when useful.
- Validate nullness, alignment, lengths, handle provenance, aliasing, integer
  conversions, and UTF-8 before entering safe domain logic.
- No panic may cross an `extern "C"` boundary. Remove panic sources and use an
  unwind guard at the outer boundary where the build supports unwinding.
- Keep declarations used by Julia and Python synchronized with the Rust
  exports. Breaking an exported signature is acceptable when all in-repository
  consumers are updated together; never allow a stale binding to call a changed
  layout or signature.
- Keep Julia and Python wrappers idiomatic but thin. Translate Rust statuses and
  optionals into native language exceptions/results and `nothing`/`None`
  consistently; never duplicate graph algorithms in bindings.
- Minimize boundary crossings with bulk APIs, but do not expose raw internal
  storage in a way that permits invariant violations or dangling views.

## Reliability and verification

For each change, add tests at the lowest appropriate layer.

- Exercise the library's generic design in its tests. Define reusable
  conformance suites that are generic over the relevant graph traits instead of
  copying the same behavioral tests for every storage type.
- Every new storage implementation must be added to all generic conformance
  suites for the capabilities it implements. Prefer a small factory or fixture
  trait when a suite needs to construct and populate graphs.
- Test generic construction, mutation, and generators through reusable suites
  parameterized by their required capability traits. Run the same scenarios for
  every compatible storage backend instead of testing generator/storage pairs
  independently.
- Tests for layout-specific associated functions should verify their
  representation-specific contract and then reuse the generic behavioral and
  invariant checks on the graph they produce.
- Write storage-specific tests only for representation-specific invariants,
  unique behavior, unsafe internals, performance characteristics, or a
  regression that the generic contract does not express.
- Keep capability suites focused: a storage type should only be required to
  pass suites for traits and guarantees it actually advertises.
- Test success, empty graphs, boundary identifiers, invalid identifiers,
  overflow, allocation/capacity errors where injectable, and state after failure.
- Use model-based or property tests for nontrivial graph mutations and
  algorithms when practical. Important invariants include symmetric incidence,
  valid indices, accurate counts, and atomic mutation on error.
- Add compile tests for generic trait combinations and `no_std` support when
  changing core APIs.
- Test the C-callable surface, including invalid pointers/handles and error
  paths, once that layer exists. Binding tests must cover error translation and
  ownership.
- Run Miri, sanitizers, fuzzing, or loom-style concurrency tests when the change
  touches unsafe code, parsers, FFI, or concurrent state and the tools apply.

Before handing off a normal Rust change, run the relevant subset of:

```text
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Also test applicable feature combinations and a representative allocator-backed
`no_std` target for changes that affect `hgraphs-core`. If a check cannot run,
state that clearly; do not claim unperformed verification.
