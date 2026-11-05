// Activates rb-sys-env: reads the ruby rbconfig from PATH and sets up linking against
// the Ruby library, so `cargo build`/`cargo test` work without rake-compiler/extconf.
fn main() {
    let _ = rb_sys_env::activate().expect("rb-sys-env: ruby not found in PATH");
}
