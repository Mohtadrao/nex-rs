# nex-rs (WIP)

A Rust port of the **nex-go** project you provided. This is a scaffolded, compiling foundation with idiomatic Rust structure and room to complete feature parity.

## Layout

```
src/
  error.rs           // error type (port of error.go)
  result_codes.rs    // constants (to be fully translated)
  counter.rs         // atomic counter (counter.go)
  rtt.rs             // round-trip-time estimator (rtt.go)
  timeout_manager.rs // lightweight timeout helpers (timeout_manager.go)
  types/             // PID and other basic types
  byte_stream.rs     // read/write helpers (byte_stream_*)
  prudp/             // PRUDP protocol (packet, server, connection)
  hpp/               // HPP protocol (packet, server, client)
```

## Next steps to complete the port

- Translate constants and enums from Go (`connection_state.go`, `result_codes`).
- Implement PRUDP packet parsing/encoding according to `prudp_packet*.go` and sliding window / retransmit logic.
- Implement HPP handshake, message framing, and RMC message encode/decode from `hpp_*.go` and `rmc_message.go`.
- Port crypto (Kerberos/RC4/Quazal) and compression if required by your use-case.
- Mirror interfaces:
  - `connection_interface.go` → Rust traits.
  - `endpoint_interface.go` → traits for callback/dispatch.
- Add tests that mirror Go tests (`*_test.go`).

This repo chooses **Tokio** for async IO and **bytes** for buffers, **thiserror**/**anyhow** for errors, and **tracing** for logs.

## Running tests locally

This project includes a suite of unit tests that exercise the PRUDP and HPP scaffolding.

Run tests with:

```bash
cargo test --workspace --all-features
```

A GitHub Actions workflow is included at `.github/workflows/rust.yml` which runs `cargo test` on pushes/PRs.


See TODO.md for current progress and remaining tasks.


## Crypto & SlidingWindow tests

Run `cargo test --test sliding_window_ported -- --nocapture` to see sliding window behavior. PRUDP crypto tests exist too.
