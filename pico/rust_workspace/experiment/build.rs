fn main() {
	println!("cargo::rerun-if-changed=../rammap.ld");
	println!("cargo::rustc-link-arg-bins=-T");
	println!("cargo::rustc-link-arg-bins=rammap.ld");
}
