use cc::Build;
use std::env;
use std::path::Path;

fn main() {
    let mut build = Build::new();

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_arch == "wasm32" && target_os == "wasi" {
        let wasi_sdk_path = env::var("WASI_SDK_PATH").expect("WASI_SDK_PATH env is missing!");

        let cc_path = Path::new(&wasi_sdk_path).join("bin").join("clang");
        let ar_path = Path::new(&wasi_sdk_path).join("bin").join("llvm-ar");
        let sysroot_path = Path::new(&wasi_sdk_path).join("share").join("wasi-sysroot");

        build
            .compiler(cc_path)
            .archiver(ar_path)
            .flag(&format!(
                "--sysroot={}",
                sysroot_path.to_str().expect("utf8")
            ))
            .include("deps/xterm-pty-wasi-polyfill");
    }

    build
        .include("deps/linenoise")
        .file("deps/linenoise/linenoise.c")
        .compile("linenoise");
}
