fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-Tboot/riscv64/src/linker.ld");
}
