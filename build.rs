fn main() {
    println!(
        "cargo:rustc-env=PROJECT_ROOT={}",
        env!("CARGO_MANIFEST_DIR")
    );
}
