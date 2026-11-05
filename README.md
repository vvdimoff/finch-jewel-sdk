# finch-jewel-sdk

Ruby-FFI glue for **Finch per-plugin crown-jewels** (Rust via [magnus]).

**Wrapper only** — no keys, no product code, no internal paths. Your plugin's jewel
crate depends on this and exposes Rust functions as `Finch::Jewels_<ns>.<name>` in Ruby,
isolated per plugin (no shared `Finch::Native`).

## Plugin jewel crate

A plugin keeps its jewels in a `jewels/` Cargo crate inside the plugin source
(`<slug>/jewels/`). It is a `cdylib`:

```toml
# <slug>/jewels/Cargo.toml
[package]
name = "finch_jewels"      # placeholder; CI overrides the crate name per ns (below)
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
finch-jewel-sdk = { git = "https://github.com/vvdimoff/finch-jewel-sdk" }
# your own algorithm deps (serde, rayon, roxmltree, …)

[build-dependencies]
rb-sys-env = "0.2"
```

```rust
// <slug>/jewels/src/lib.rs
use finch_jewel_sdk::magnus::{self, function, Error, RString, Ruby};

fn detect_batch(ruby: &Ruby, packed: RString) -> Result<RString, Error> {
    // … pure algorithm; bulk numbers via finch_jewel_sdk::{pack_f64, unpack_f64} …
    Ok(RString::from_slice(&[]))
}

finch_jewel_sdk::export! {
    "detect_batch" => detect_batch, 1;   // "<ruby_name>" => <rust_fn>, <arity>;
}
```

The Ruby side calls **only its own module**, `Finch::Jewels_<ns>.detect_batch(...)` — never
a shared `Finch::Native`.

## How the per-ns binary is produced (CI)

Ruby derives the Init symbol from the `.so` basename, and the module name from
`FINCH_JEWEL_NS`. CI therefore builds with both, and asserts they match:

```sh
NS=addon_blum          # <ns>: <slug> for a plugin, addon_<slug> for an addon
test "$CRATE_NAME" = "finch_jewels_${NS}" || { echo "crate-name must be finch_jewels_$NS"; exit 1; }
FINCH_JEWEL_NS="$NS" cargo rustc --release --crate-name "finch_jewels_${NS}" -- ...
# → finch_jewels_<ns>.<ext> with Init_finch_jewels_<ns> and module Finch::Jewels_<ns>
```

The `.rbz` carries the binary at `finch_jewels/v1/<rv>/finch_jewels_<ns>.<ext>`
(mirrors the native layout). The Finch loader picks it up per ns.

Nothing from `<slug>/jewels/` (`.rs`, `Cargo.*`, `target/`, `jewels.lock`) ships in the
`.rbz` — only the compiled binary.

## Data passing

Pass Ruby-native types (`Array`/`Hash`/`String`/`Integer`/`Float`) directly. For hot
paths with large numeric arrays, pass one packed String instead of per-element
conversion (no msgpack): [`pack_f64`]/[`unpack_f64`] here, `Array#pack('E*')` /
`String#unpack('E*')` on the Ruby side.

[magnus]: https://github.com/matsadler/magnus
