use std::env;
use std::fs::create_dir;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let glslang_dir = env::var("GLSLANG_TARGET_DIR")
        .expect("required environment variable GLSLANG_TARGET_DIR is not provided");
    let realcugan_dir = out_dir.join("realcugan");
    create_dir(&realcugan_dir).unwrap_or_default();
    let realcugan = {
        let mut config = Config::new("src/");
        config
            .out_dir(realcugan_dir)
            .define("GLSLANG_TARGET_DIR", glslang_dir);
        config.build()
    };
    println!("cargo:rustc-link-search=native={}", realcugan.join("lib").display());
    println!("cargo:rustc-link-lib=static:-bundle={}", "realcugan-ncnn-vulkan-wrapper");
    println!("cargo:rustc-link-lib=dylib=ncnn");
    if cfg!(unix) {
        println!("cargo:rustc-link-lib=dylib={}", "stdc++");
    }
}