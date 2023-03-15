use std::{env, path::{PathBuf, Path}};

extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=../libasm.a");
    println!("cargo:rerun-if-changed=bindgen/libasm.h");
    println!("cargo:rerun-if-changed=utils/comparison.c");
    println!("cargo:rerun-if-changed=utils/no_free.c");
    println!(r"cargo:rustc-link-search=..");
    println!(r"cargo:rustc-link-lib=asm");

    if !Path::new("../libasm.a").exists() {
        panic!("`../libasm.a` does not exist.")
    }

    cc::Build::new()
        .file("utils/comparison.c")
        .file("utils/no_free.c")
        .compile("tester_utils");

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
