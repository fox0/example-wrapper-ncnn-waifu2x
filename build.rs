extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=ncnn-20210720-ubuntu-1804/lib");
    for i in [
        "GenericCodeGen",
        "glslang",
        "MachineIndependent",
        "OGLCompiler",
        "OSDependent",
        "SPIRV",
        //
        "ncnn",
        //
        "stdc++",
        "gomp",
    ] {
        println!("cargo:rustc-link-lib=static={}", i);
    }
    println!("cargo:rustc-link-lib=dylib=vulkan"); // apt install libvulkan-dev
    println!("cargo:rerun-if-changed=wrapper.h");

    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from("src").join("bindings.rs"))
        .unwrap();
}
