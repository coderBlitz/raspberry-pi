fn main() {
	println!("cargo::rerun-if-changed=../bootmap.ld");
	println!("cargo::rustc-link-arg-bins=-T");
	println!("cargo::rustc-link-arg-bins=bootmap.ld");
}
