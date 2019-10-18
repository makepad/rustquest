use std::env;
use std::path::PathBuf;

fn main() {
    let android_ndk_home = PathBuf::from(env::var("ANDROID_NDK_HOME").unwrap());
    println!(
        "cargo:rustc-link-search={}",
        android_ndk_home
            .join("platforms/android-26/arch-arm64/usr/lib")
            .to_str()
            .unwrap()
    );
    println!("cargo:rustc-link-lib=log");
    let bindings = bindgen::Builder::default()
        .clang_arg(format!(
            "-I{}",
            android_ndk_home
                .join("toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include")
                .to_str()
                .unwrap()
        ))
        .header("wrapper.h")
        .generate()
        .expect("can't generate bindings");
    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("can't write bindings");
}
