extern crate cmake;

fn main() {
    let dst = cmake::build("openvr");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=openvr_api");

    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-lib=stdc++");
}
