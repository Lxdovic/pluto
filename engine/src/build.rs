fn main() {
    if std::env::var("CARGO_FEATURE_TUNING").is_ok() {
        println!("cargo:rustc-cfg=feature=\"tuning\"");
    }

    // pluto classical means no use of NNUE, HCE is used instead
    if std::env::var("CARGO_FEATURE_CLASSICAL").is_ok() {
        println!("cargo:rustc-cfg=feature=\"classical\"");
    }
}
