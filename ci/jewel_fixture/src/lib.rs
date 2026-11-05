//! CI fixture — the README `export!` example, compiled and then LOADED in Ruby by
//! .github/workflows/ci.yml (see ../verify.rb). Proves the full path: `export!` macro →
//! `Init_finch_jewels_ci_example` symbol → module `Finch::Jewels_ci_example` → callable
//! methods at arity 1 AND 2. "Проверено, а не сказано."
//!
//! A jewel imports only the types its own fn signatures name — `export!` fully-qualifies
//! magnus internally.
use finch_jewel_sdk::magnus::{Error, RString, Ruby};
use finch_jewel_sdk::pack_f64;

// arity 1
fn triple(_ruby: &Ruby, x: f64) -> Result<f64, Error> {
    Ok(x * 3.0)
}

// arity 2 — повертає запакований f64-рядок (exercises pack_f64 + RString out).
fn pack_pair(ruby: &Ruby, a: f64, b: f64) -> Result<RString, Error> {
    Ok(ruby.str_from_slice(&pack_f64(&[a, b])))
}

finch_jewel_sdk::export! {
    "triple"    => triple,    1;
    "pack_pair" => pack_pair, 2;
}
