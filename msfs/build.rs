fn main() {
    let wasm = std::env::var("TARGET").unwrap().starts_with("wasm32-");
    let msfs_sdk = std::env::var("MSFS_SDK").unwrap_or_else(calculate_msfs_sdk_path);
    println!("Found MSFS SDK: {:?}", msfs_sdk);

    println!("cargo:rerun-if-changed=src/bindgen_support/wrapper.h");
    let mut bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", msfs_sdk))
        .clang_arg(format!("-I{}", "src/bindgen_support"))
        .clang_arg("-fms-extensions")
        .clang_arg("-fvisibility=default")
        .clang_arg("-xc++")
        .clang_arg("-v")
        .header("src/bindgen_support/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustified_enum("SIMCONNECT_EXCEPTION")
        .impl_debug(true);

    if wasm {
        bindings = bindings.clang_arg("-D_MSFS_WASM 1");
    }

    bindings
        .generate()
        .unwrap()
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
        )
        .unwrap();

    if !wasm {
        println!(
            "cargo:rustc-link-search={}/SimConnect SDK/lib/static",
            msfs_sdk
        );
        println!("cargo:rustc-link-lib=SimConnect");
        println!("cargo:rustc-link-lib=shlwapi");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=shell32");
    }
}

fn calculate_msfs_sdk_path(_: std::env::VarError) -> String {
    for p in ["/mnt/c/MSFS SDK", r"C:\MSFS SDK"].iter() {
        if std::path::Path::new(p).exists() {
            return p.to_string();
        }
    }
    panic!("Could not locate MSFS SDK. Make sure you have it installed or try setting the MSFS_SDK env var.");
}
