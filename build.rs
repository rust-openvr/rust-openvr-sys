extern crate cmake;

fn main() {
    let mut cfg = cmake::Config::new("openvr");

    // Work around broken cmake build
    #[cfg(windows)]
    cfg.cxxflag("/DWIN32");

    #[cfg(target_os="macos")]
    cfg.define("BUILD_UNIVERSAL", "OFF");

    let dst = cfg.build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    #[cfg(all(windows, target_pointer_width = "64"))]
    println!("cargo:rustc-link-lib=static=openvr_api64");

    #[cfg(not(all(windows, target_pointer_width = "64")))]
    println!("cargo:rustc-link-lib=static=openvr_api");

    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-lib=stdc++");

    #[cfg(target_os="macos")]
    println!("cargo:rustc-link-lib=c++");
}
