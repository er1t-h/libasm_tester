fn main() {
	println!("cargo:rerun-if-changed=libasm.a");
	println!(r"cargo:rustc-link-search=.");
	println!(r"cargo:rustc-link-lib=asm")
}
