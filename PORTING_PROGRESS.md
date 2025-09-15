
# Porting progress snapshot
Generated: 2025-09-04T18:25:32.936165Z

This file lists high-level remaining work and the generated test scaffold `src/tests/ported_go_tests_scaffold.rs` which creates ignored
placeholders for each Go test file found in the uploaded nex-go project. Use that scaffold as a checklist and update individual tests as you port them.

Top remaining targets:
- PRUDP exact packet formats and all prudp_packet_* variants
- sliding_window.go full parity (NAK policies, NAK suppression windows, multi-ack interactions)
- packet_dispatch_queue.go exact behavior (CONNECT ordering, priorities)
- hpp/rmc: full message structures, call/response correlation
- kerberos.go and comprehensive crypto + compression port
- run cargo test and fix failing tests iteratively (CI added)

