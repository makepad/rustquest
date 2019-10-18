use std::env;
use std::path::PathBuf;

fn main() {
    let android_ndk_home = PathBuf::from(env::var("ANDROID_NDK_HOME").unwrap());
    let ovr_home = PathBuf::from(env::var("OVR_HOME").unwrap());
    println!(
        "cargo:rustc-link-search={}",
        ovr_home
            .join("VrApi/Libs/Android/arm64-v8a/Debug")
            .to_str()
            .unwrap()
    );
    println!("cargo:rustc-link-lib=vrapi");
    let bindings = bindgen::Builder::default()
        .clang_arg(format!(
            "-I{}",
            android_ndk_home
                .join("toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include")
                .to_str()
                .unwrap()
        ))
        .clang_arg(format!(
            "-I{}",
            ovr_home.join("VrApi/Include").to_str().unwrap()
        ))
        .header("wrapper.h")
        .generate()
        .expect("can't generate bindings");
    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("can't write bindings");
}
