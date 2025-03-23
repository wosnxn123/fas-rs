use std::fs::copy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 部署Zygisk共享库
    copy("module/zygisk/libzygisk.so", "target/arm64-v8a/libzygisk.so")?;
    println!("cargo:rustc-link-search=native=target/arm64-v8a");
    println!("cargo:rustc-link-lib=zygisk");

    // 添加Android NDK编译标志
    println!("cargo:rustc-cfg=android");
    println!("cargo:rustc-flags=-L target/arm64-v8a");
}
