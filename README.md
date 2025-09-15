# nex-rs (WIP)

A Rust port of the **nex-go** project you provided. This is a scaffolded, compiling foundation with idiomatic Rust structure and room to complete feature parity.

## Layout

```
src/
    auth/ // authentication-related modules
    bin/ // example binaries (e.g. nex_server)
    compression/ // zlib/LZO and other compression stubs
    constants/ // enums and protocol constants
    crypto/ // RC4, Quazal, Kerberos stubs
    hpp/ // HPP protocol (packet, server, client)
    prudp/ // PRUDP protocol (packet, server, connection)
    services/ // service skeletons (auth, etc.)
    tests/ // Go → Rust test scaffolding
    types/ // PID and other basic types
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


## Running Commands
-cargo clean
-cargo build (contain warning, 0 errors)
-cargo test (contain warning, 0 errors)
-cargo run

## Contributing

Contributions are welcome! 🎉

Feel free to fork this repo, make your own branches, and upload code publicly.

Submit pull requests for new features, bug fixes, or improvements.

Check the TODO.md
 and PORT_TODO.md
 for areas where help is especially needed.

Please keep commits clear and focused — small PRs are easier to review.

Whether you’re fixing a bug, adding documentation, or porting another Go file to Rust, your help is appreciated.



Credits

Original Go implementation: PretendoNetwork/nex-go

Rust port: this project (nex-rs)