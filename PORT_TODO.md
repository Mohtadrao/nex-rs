# nex-rs Port Status (auto-generated)

This file lists **Go files** that do not yet have a direct Rust counterpart in this port.

Items with ✅ were stubbed in this pass; others remain TODO.


## Stubbed now

- ✅ `constants/*` (core enums)
- ✅ `connection_state.go`
- ✅ `account.go`
- ✅ `compression/{algorithm,dummy,zlib}.go` (zlib behind `zlib` feature)
- ✅ Added `types::pid::PID`


## Remaining Go files without matching Rust files

- byte_stream_in.go  → (expected roughly at `src/byte_stream_in.rs`)
- byte_stream_out.go  → (expected roughly at `src/byte_stream_out.rs`)
- byte_stream_settings.go  → (expected roughly at `src/byte_stream_settings.rs`)
- connection_interface.go  → (expected roughly at `src/connection_interface.rs`)
- encryption/algorithm.go  → (expected roughly at `src/algorithm.rs`)
- encryption/dummy.go  → (expected roughly at `src/dummy.rs`)
- encryption/quazal_rc4.go  → (expected roughly at `src/quazal_rc4.rs`)
- encryption/rc4.go  → (expected roughly at `src/rc4.rs`)
- endpoint_interface.go  → (expected roughly at `src/endpoint_interface.rs`)
- hpp_client.go  → (expected roughly at `src/hpp_client.rs`)
- hpp_packet.go  → (expected roughly at `src/hpp_packet.rs`)
- hpp_server.go  → (expected roughly at `src/hpp_server.rs`)
- init.go  → (expected roughly at `src/init.rs`)
- kerberos.go  → (expected roughly at `src/kerberos.rs`)
- kerberos_test.go  → (expected roughly at `src/kerberos_test.rs`)
- library_version.go  → (expected roughly at `src/library_version.rs`)
- mutex_map.go  → (expected roughly at `src/mutex_map.rs`)
- mutex_slice.go  → (expected roughly at `src/mutex_slice.rs`)
- packet_dispatch_queue.go  → (expected roughly at `src/packet_dispatch_queue.rs`)
- packet_dispatch_queue_test.go  → (expected roughly at `src/packet_dispatch_queue_test.rs`)
- packet_interface.go  → (expected roughly at `src/packet_interface.rs`)
- prudp_connection.go  → (expected roughly at `src/prudp_connection.rs`)
- prudp_endpoint.go  → (expected roughly at `src/prudp_endpoint.rs`)
- prudp_packet.go  → (expected roughly at `src/prudp_packet.rs`)
- prudp_packet_interface.go  → (expected roughly at `src/prudp_packet_interface.rs`)
- prudp_packet_lite.go  → (expected roughly at `src/prudp_packet_lite.rs`)
- prudp_packet_v0.go  → (expected roughly at `src/prudp_packet_v0.rs`)
- prudp_packet_v1.go  → (expected roughly at `src/prudp_packet_v1.rs`)
- prudp_server.go  → (expected roughly at `src/prudp_server.rs`)
- prudp_v0_settings.go  → (expected roughly at `src/prudp_v0_settings.rs`)
- prudp_v1_settings.go  → (expected roughly at `src/prudp_v1_settings.rs`)
- rmc_message.go  → (expected roughly at `src/rmc_message.rs`)
- service_protocol.go  → (expected roughly at `src/service_protocol.rs`)
- sliding_window.go  → (expected roughly at `src/sliding_window.rs`)
- socket_connection.go  → (expected roughly at `src/socket_connection.rs`)
- stream_settings.go  → (expected roughly at `src/stream_settings.rs`)
- sum.go  → (expected roughly at `src/sum.rs`)
- test/auth.go  → (expected roughly at `src/auth.rs`)
- test/generate_ticket.go  → (expected roughly at `src/generate_ticket.rs`)
- test/hpp.go  → (expected roughly at `src/hpp.rs`)
- test/main.go  → (expected roughly at `src/main.rs`)
- test/secure.go  → (expected roughly at `src/secure.rs`)
- timeout.go  → (expected roughly at `src/timeout.rs`)
- types/any_object_holder.go  → (expected roughly at `src/any_object_holder.rs`)
- types/bool.go  → (expected roughly at `src/bool.rs`)
- types/buffer.go  → (expected roughly at `src/buffer.rs`)
- types/class_version_container.go  → (expected roughly at `src/class_version_container.rs`)
- types/data.go  → (expected roughly at `src/data.rs`)
- types/data_holder.go  → (expected roughly at `src/data_holder.rs`)
- types/datetime.go  → (expected roughly at `src/datetime.rs`)
- types/double.go  → (expected roughly at `src/double.rs`)
- types/float.go  → (expected roughly at `src/float.rs`)
- types/int16.go  → (expected roughly at `src/int16.rs`)
- types/int32.go  → (expected roughly at `src/int32.rs`)
- types/int64.go  → (expected roughly at `src/int64.rs`)
- types/int8.go  → (expected roughly at `src/int8.rs`)
- types/list.go  → (expected roughly at `src/list.rs`)
- types/map.go  → (expected roughly at `src/map.rs`)
- types/pid.go  → (expected roughly at `src/pid.rs`)
- types/qbuffer.go  → (expected roughly at `src/qbuffer.rs`)
- types/qresult.go  → (expected roughly at `src/qresult.rs`)
- types/quuid.go  → (expected roughly at `src/quuid.rs`)
- types/readable.go  → (expected roughly at `src/readable.rs`)
- types/result_range.go  → (expected roughly at `src/result_range.rs`)
- types/rv_connection_data.go  → (expected roughly at `src/rv_connection_data.rs`)
- types/rv_type.go  → (expected roughly at `src/rv_type.rs`)
- types/station_url.go  → (expected roughly at `src/station_url.rs`)
- types/string.go  → (expected roughly at `src/string.rs`)
- types/structure.go  → (expected roughly at `src/structure.rs`)
- types/uint16.go  → (expected roughly at `src/uint16.rs`)
- types/uint32.go  → (expected roughly at `src/uint32.rs`)
- types/uint64.go  → (expected roughly at `src/uint64.rs`)
- types/uint8.go  → (expected roughly at `src/uint8.rs`)
- types/variant.go  → (expected roughly at `src/variant.rs`)
- types/writable.go  → (expected roughly at `src/writable.rs`)
- virtual_port.go  → (expected roughly at `src/virtual_port.rs`)
- websocket_server.go  → (expected roughly at `src/websocket_server.rs`)
## This pass - additional stubs added

- ✅ `byte_stream_in.go` -> `src/byte_stream_in.rs`
- ✅ `byte_stream_out.go` -> `src/byte_stream_out.rs`
- ✅ `byte_stream_settings.go` -> `src/byte_stream_settings.rs`
- ✅ `connection_interface.go` -> `src/connection_interface.rs`
- ✅ `encryption/algorithm.go` -> `src/algorithm.rs`
- ✅ `encryption/dummy.go` -> `src/dummy.rs`

## This pass - more stubs

- ✅ `types/uint8.go` -> `src/types/uint8.rs`
- ✅ `types/uint64.go` -> `src/types/uint64.rs`
- ✅ `types/variant.go` -> `src/types/variant.rs`
- ✅ `types/writable.go` -> `src/types/writable.rs`
- ✅ `virtual_port.go` -> `src/virtual_port.rs`
- ✅ `websocket_server.go` -> `src/websocket_server.rs` (placeholder)

## This pass - auto-created stubs for remaining Go files

- ✅ `quazal_rc4.rs` (stub created)
- ✅ `rc4.rs` (stub created)
- ✅ `endpoint_interface.rs` (stub created)
- ✅ `hpp_client.rs` (stub created)
- ✅ `hpp_packet.rs` (stub created)
- ✅ `hpp_server.rs` (stub created)
- ✅ `init.rs` (stub created)
- ✅ `kerberos.rs` (stub created)
- ✅ `kerberos_test.rs` (stub created)
- ✅ `library_version.rs` (stub created)
- ✅ `mutex_map.rs` (stub created)
- ✅ `mutex_slice.rs` (stub created)
- ✅ `packet_dispatch_queue.rs` (stub created)
- ✅ `packet_dispatch_queue_test.rs` (stub created)
- ✅ `packet_interface.rs` (stub created)
- ✅ `prudp_connection.rs` (stub created)
- ✅ `prudp_endpoint.rs` (stub created)
- ✅ `prudp_packet.rs` (stub created)
- ✅ `prudp_packet_interface.rs` (stub created)
- ✅ `prudp_packet_lite.rs` (stub created)
- ✅ `prudp_packet_v0.rs` (stub created)
- ✅ `prudp_packet_v1.rs` (stub created)
- ✅ `prudp_server.rs` (stub created)
- ✅ `prudp_v0_settings.rs` (stub created)
- ✅ `prudp_v1_settings.rs` (stub created)
- ✅ `rmc_message.rs` (stub created)
- ✅ `service_protocol.rs` (stub created)
- ✅ `sliding_window.rs` (stub created)
- ✅ `socket_connection.rs` (stub created)
- ✅ `stream_settings.rs` (stub created)
- ✅ `sum.rs` (stub created)
- ✅ `auth.rs` (stub created)
- ✅ `generate_ticket.rs` (stub created)
- ✅ `hpp.rs` (stub created)
- ✅ `main.rs` (stub created)
- ✅ `secure.rs` (stub created)
- ✅ `timeout.rs` (stub created)
- ✅ `types/any_object_holder.rs` (stub created)
- ✅ `types/bool.rs` (stub created)
- ✅ `types/buffer.rs` (stub created)
- ✅ `types/class_version_container.rs` (stub created)
- ✅ `types/data.rs` (stub created)
- ✅ `types/data_holder.rs` (stub created)
- ✅ `types/datetime.rs` (stub created)
- ✅ `types/double.rs` (stub created)
- ✅ `types/float.rs` (stub created)
- ✅ `types/int16.rs` (stub created)
- ✅ `types/int32.rs` (stub created)
- ✅ `types/int64.rs` (stub created)
- ✅ `types/int8.rs` (stub created)
- ✅ `types/list.rs` (stub created)
- ✅ `types/map.rs` (stub created)
- ✅ `types/qbuffer.rs` (stub created)
- ✅ `types/qresult.rs` (stub created)
- ✅ `types/quuid.rs` (stub created)
- ✅ `types/readable.rs` (stub created)
- ✅ `types/result_range.rs` (stub created)
- ✅ `types/rv_connection_data.rs` (stub created)
- ✅ `types/rv_type.rs` (stub created)
- ✅ `types/station_url.rs` (stub created)
- ✅ `types/string.rs` (stub created)
- ✅ `types/structure.rs` (stub created)
- ✅ `types/uint16.rs` (stub created)
- ✅ `types/uint32.rs` (stub created)

## Skipped (already present)

- account.rs
- byte_stream_in.rs
- byte_stream_out.rs
- byte_stream_settings.rs
- compression/algorithm.rs
- compression/dummy.rs
- compression/lzo.rs
- compression/zlib.rs
- connection_interface.rs
- connection_state.rs
- constants/nat_filtering_properties.rs
- constants/nat_mapping_properties.rs
- constants/prudp_packet_flags.rs
- constants/prudp_packet_types.rs
- constants/signature_method.rs
- constants/station_url_flag.rs
- constants/station_url_type.rs
- constants/stream_type.rs
- counter.rs
- algorithm.rs
- dummy.rs
- error.rs
- result_codes.rs
- rtt.rs
- timeout_manager.rs
- types/pid.rs
- types/uint64.rs
- types/uint8.rs
- types/variant.rs
- types/writable.rs
- virtual_port.rs
- websocket_server.rs

## This pass - PRUDP dispatch, fragmentation, retransmit+keepalive added

- ✅ `packet_dispatch_queue.rs` implemented
- ✅ `fragmentation.rs` helper added
- ✅ `connection.rs` extended with `spawn_retransmit_and_keepalive` logic (uses existing methods `build_prudp_data_packet_bytes` and `send_ping` if present)


## This pass - RV, RMC, services, rtt, crypto stubs, examples added

- ✅ `rv.rs` session manager
- ✅ `rmc.rs` basic dispatcher stub
- ✅ `services::auth` skeleton
- ✅ `rtt.rs` simple estimator
- ✅ `crypto::stub` helper
- ✅ `examples/server.rs` and `examples/client.rs` skeletons

