// Wire up ruby rbconfig for magnus/rb-sys at build (mirror of the SDK's own build.rs и
// того, що робить реальний jewel-крейт).
fn main() {
    let _ = rb_sys_env::activate().expect("rb-sys-env activate (ruby headers on PATH?)");
}
