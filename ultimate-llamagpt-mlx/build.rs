fn main() {
    println!("cargo:rerun-if-changed=src/ml/bindings.h");
    println!("cargo:rerun-if-changed=src/ml/mlx_wrapper.cpp");
    #[cfg(feature = "apple-silicon")]
    {
        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
        if target_os == "macos" {
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=MetalPerformanceShaders");
            cxx_build::bridge("src/ml/bindings.rs")
                .file("src/ml/mlx_wrapper.cpp")
                .flag_if_supported("-std=c++17")
                .flag_if_supported("-Wno-unused-parameter")
                .compile("mlx_wrapper");
        }
    }
}
