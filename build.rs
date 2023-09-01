use std::env;

fn main() {
    println!("cargo:rerun-if-changed=platform/*");

    let target = env::var("TARGET").unwrap();
    println!("cargo:rustc-link-arg=-Tplatform/{}/linker.ld", target);
}