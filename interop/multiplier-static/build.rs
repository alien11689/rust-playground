use std::env;
use std::path::Path;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("../adder/adder.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("./bindings.rs"))
        .expect("Couldn't write bindings!");

    // Linkowanie do statycznej biblioteki C
    println!("cargo:rerun-if-changed=../adder/adder.h");
    println!("cargo:rustc-link-lib=static=adder");
    println!("cargo:rustc-link-search=../adder");
}
