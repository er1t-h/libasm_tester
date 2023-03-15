use std::{
    env,
    path::{Path, PathBuf},
};

extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=../libasm.a");
    println!("cargo:rerun-if-changed=bindgen/libasm.h");
    println!("cargo:rerun-if-changed=utils/comparison.c");
    println!("cargo:rerun-if-changed=utils/no_free.c");

    // Path to the libasm.a file (equivalent to -L flag when using cc)
    println!(r"cargo:rustc-link-search=..");
    // Name of the library (equivalent to -l flag when using cc)
    println!(r"cargo:rustc-link-lib=asm");

    // You can remove this if your libasm.a is not located in the folder above
    // But errors tend to be incomprehensible if you give the wrong path
    if !Path::new("../libasm.a").exists() {
        panic!("`../libasm.a` does not exist.")
    }

    // Builds the helper functions
    cc::Build::new()
        .file("utils/comparison.c")
        .file("utils/no_free.c")
        .compile("tester_utils");

    // Creates raw bindings to use our ASM library in a Rust code
    let bindings = bindgen::Builder::default()
        .header("bindgen/libasm.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
