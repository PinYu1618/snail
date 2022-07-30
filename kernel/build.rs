fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("riscv64") {
        println!("cargo:rustc-cfg=riscv64");
        println!("cargo:rustc-link-arg=-Tkernel/src/platform/riscv64/linker.ld");
    }
}