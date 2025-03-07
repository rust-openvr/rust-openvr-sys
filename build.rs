use bindgen;
use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Missing OUT_DIR env var"));

    println!("cargo:rerun-if-changed=wrapper.hpp");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();

    // Configure cmake to place build output in OUT_DIR
    let out_dir_str = out_dir.to_string_lossy().into_owned();
    let mut config = cmake::Config::new("openvr");
    let config = config
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", &out_dir_str)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", &out_dir_str)
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY", &out_dir_str)
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY_DEBUG", &out_dir_str)
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY_RELEASE", &out_dir_str)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_DEBUG", &out_dir_str)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_RELEASE", &out_dir_str)
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG", &out_dir_str)
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE", &out_dir_str)
        .out_dir(&out_dir); 

    if target_os == "macos" {
        config.define("BUILD_UNIVERSAL", "OFF");
    } else if target_os == "windows" {
        // Work around broken cmake build.
        config.cxxflag("/DWIN32");
    }

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    if target_os == "windows" && target_pointer_width == "64" {
        println!("cargo:rustc-link-lib=static=openvr_api64");
    } else {
        println!("cargo:rustc-link-lib=static=openvr_api");
    }

    if target_os == "linux" {
        println!("cargo:rustc-link-lib=stdc++");
    } else if target_os == "macos" {
        println!("cargo:rustc-link-lib=c++");
    } else if target_os == "windows" {
        println!("cargo:rustc-link-lib=shell32");
    }

    // Generate bindings and write them into OUT_DIR
    bindgen::builder()
        .header("wrapper.hpp")
        .constified_enum(".*")
        .prepend_enum_name(false)
        .derive_default(true)
        .generate()
        .expect("could not generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("could not write bindings.rs");
}
