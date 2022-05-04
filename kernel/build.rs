fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("riscv64") {
        println!("cargo:rustc-cfg=riscv");
        println!("cargo:rustc-cfg=riscv64");
        println!("cargo:rustc-link-arg=-Tkernel/src/linker.ld");
    } else if target.contains("x86_64") {
        println!("cargo:rustc-cfg=x86_64");
    }
}
