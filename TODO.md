# nex-rs Port — PROGRESS & TODO

Last updated: 2025-09-04T18:22:21.155777Z

## What has been completed (major milestones)
- Project unzipped and analyzed from provided Go repo.
- Scaffolded a Rust crate (`nex-rs`) with Cargo.toml and modules.
- Implemented core modules: PRUDP packet parsing (v0/v1 stub), sliding window, connection retransmit loop, PRUDP server.
- Implemented HPP framing, simple RMC parser, dispatcher with method registry, HPP server & client skeletons.
- Added auth stubs and implemented RC4 + Quazal wrapper, and Kerberos derive KDF.
- Created many unit tests and an integration HPP end-to-end test.
- Added example binary `src/bin/nex_server.rs` and CI workflow `.github/workflows/rust.yml`.
- Iteratively refined sliding window (NAK/backoff), per-packet timestamps, RTT estimator integration, and retransmit logic.

## Remaining high-priority tasks
1. **Full PRUDP parity (HIGH)**: port all PRUDP packet variants, exact bit layouts, multi-ACK/NAK semantics, edge cases from `prudp_packet_*.go` and `sliding_window.go`.
2. **PacketDispatchQueue parity (MED)**: ensure all behaviors of Go queue (CONNECT packet special-casing, reordering rules) match exactly (we ported a basic version).
3. **HPP / RMC full semantics (HIGH)**: implement full RMC message structures, method call/response correlation, error codes, fragmentation if present.
4. **Kerberos & Crypto parity (HIGH)**: port full `kerberos.go`, Quazal/RC4 exact behavior, and any compression (zlib/LZO) used by the original repo.
5. **Tests parity (HIGH)**: port remaining Go tests line-by-line and ensure Rust tests reproduce Go test expectations.
6. **Interop & Integration tests (HIGH)**: fuzz/verify against a known-good NEX implementation or capture traces to ensure binary compatibility.
7. **Performance & Concurrency fixes (MED)**: tune Tokio usage, avoid blocking, add backpressure, and optimize packet dispatch paths.
8. **Documentation & examples (LOW)**: add usage docs, examples for client/server flows, and publish crate layout.

## Why this takes time
- The original Go project is a complete network protocol stack with subtle binary formats and timing behavior; correct porting requires per-field parity and thorough testing.
- Concurrency, timers, and retransmit logic are timing-sensitive; replicating semantics in Rust (`tokio`) needs careful per-packet timestamping and tests to avoid regressions.
- Cryptography and authentication (Kerberos, RC4) must be exact for interoperability — a small change will break compatibility.
- I am implementing this incrementally with tests and integration checks to ensure correctness rather than a quick superficial translation which would likely fail interoperability tests.

## What I'm doing now
- Continuing to port remaining PRUDP behaviors and Go tests (priority: full PRUDP parity -> HPP/RMC -> Kerberos/crypto).
- I will add more unit tests mirroring Go tests and integration tests next.

If you'd like to reprioritize (for example: "stop crypto, finish HPP end-to-end first"), tell me and I'll switch focus immediately.