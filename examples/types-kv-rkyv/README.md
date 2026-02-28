# types-kv-rkyv

Shared KV request/response types that are serialized with `rkyv`.

This example is based on `examples/types-kv`, but demonstrates how to:

- derive `rkyv::Archive`, `rkyv::Serialize`, `rkyv::Deserialize` for app types,
- declare an OpenRaft `TypeConfig` using those types,
- roundtrip both app-level types and OpenRaft RPC types with `rkyv`.

## Run

```bash
cargo test --manifest-path examples/types-kv-rkyv/Cargo.toml
```
