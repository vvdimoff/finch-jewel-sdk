//! finch-jewel-sdk — Ruby-FFI glue for Finch per-plugin crown-jewels (magnus).
//!
//! WRAPPER ONLY: no keys, no product code, no internal paths. A plugin's jewel crate
//! (a `cdylib`) depends on this and calls [`export!`] to expose Rust functions as
//! `Finch::Jewels_<ns>.<name>` in Ruby.
//!
//! Per-ns Init + module (mirrors the per-slug native, `-DFINCH_SLUG`-style strictness):
//! CI builds the plugin crate with `--crate-name finch_jewels_<ns>` (so magnus generates
//! the `Init_finch_jewels_<ns>` symbol that Ruby derives from the `.so` basename) and sets
//! the env `FINCH_JEWEL_NS=<ns>` (which [`export!`] reads via `env!` AT PLUGIN BUILD TIME,
//! so the module is `Finch::Jewels_<ns>`). CI MUST assert
//! `crate-name == "finch_jewels_${FINCH_JEWEL_NS}"`, else fail the build.
//!
//! The SDK itself defines no `Finch::Native` and no process-global state — each plugin's
//! jewels live only under its own `Finch::Jewels_<ns>`.

pub use magnus;

/// Bulk `f64` → little-endian bytes. Pass a large numeric array to Ruby as ONE String
/// instead of element-by-element magnus conversion (and without msgpack). On the Ruby
/// side: `arr.pack('E*')` to send, `str.unpack('E*')` to receive.
pub fn pack_f64(v: &[f64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len() * 8);
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
    out
}

/// Inverse of [`pack_f64`]: little-endian bytes → `Vec<f64>` (trailing partial chunk ignored).
pub fn unpack_f64(b: &[u8]) -> Vec<f64> {
    b.chunks_exact(8)
        .map(|c| f64::from_le_bytes(c.try_into().unwrap()))
        .collect()
}

/// Define (idempotently) and return the per-ns module `Finch::Jewels_<ns>`.
///
/// `ns` MUST be supplied by the plugin crate (so `env!("FINCH_JEWEL_NS")` is evaluated in
/// the plugin's build, not the SDK's). [`export!`] does this for you.
pub fn jewel_module(ruby: &magnus::Ruby, ns: &str) -> Result<magnus::RModule, magnus::Error> {
    use magnus::Module;
    assert!(!ns.is_empty(), "finch-jewel-sdk: FINCH_JEWEL_NS must be non-empty");
    let finch = ruby.define_module("Finch")?;
    // magnus 0.7: IntoId не реалізований для &String — передаємо String за значенням.
    finch.define_module(format!("Jewels_{ns}"))
}

/// Expose Rust functions as `Finch::Jewels_<ns>.<ruby_name>`.
///
/// Expands in the PLUGIN crate, so `env!("FINCH_JEWEL_NS")` reads the plugin's build env
/// and `#[magnus::init]` emits the `Init_<crate-name>` symbol. Each entry is
/// `"<ruby_name>" => <rust_fn>, <arity>;` (magnus needs the arity).
///
/// ```ignore
/// // A jewel imports only the types its own signatures name; export! fully-qualifies magnus.
/// use finch_jewel_sdk::magnus::{Error, RString, Ruby};
///
/// fn detect_batch(ruby: &Ruby, packed: RString) -> Result<RString, Error> { /* … */ }
///
/// finch_jewel_sdk::export! {
///     "detect_batch" => detect_batch, 1;
/// }
/// ```
#[macro_export]
macro_rules! export {
    ( $( $ruby_name:literal => $rust_fn:path , $arity:tt );+ $(;)? ) => {
        #[$crate::magnus::init]
        fn __finch_jewel_init(ruby: &$crate::magnus::Ruby) -> ::core::result::Result<(), $crate::magnus::Error> {
            // Module::define_module + Object::define_singleton_method — обидва трейти потрібні.
            use $crate::magnus::{Module, Object};
            // env! evaluated HERE, in the plugin crate, at the plugin's build.
            let ns: &str = env!(
                "FINCH_JEWEL_NS",
                "finch-jewel-sdk: set FINCH_JEWEL_NS=<ns> when building the jewel crate",
            );
            let m = $crate::jewel_module(ruby, ns)?;
            $(
                m.define_singleton_method($ruby_name, $crate::magnus::function!($rust_fn, $arity))?;
            )+
            Ok(())
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_roundtrip() {
        let v = vec![1.0_f64, -2.5, 3.14159, 0.0, 1e300];
        assert_eq!(unpack_f64(&pack_f64(&v)), v);
    }

    #[test]
    fn unpack_ignores_partial_tail() {
        let mut b = pack_f64(&[1.0, 2.0]);
        b.push(0xAB); // stray trailing byte
        assert_eq!(unpack_f64(&b), vec![1.0, 2.0]);
    }
}
