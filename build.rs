extern crate rustc_serialize;
use rustc_serialize::base64::FromBase64;

use std::env;

fn main() {
    let codegen_blob = env::var("DEP_TEENSYRS_CODEGEN").unwrap();
    let codegen = String::from_utf8(codegen_blob.from_base64().unwrap()).unwrap();
    println!("cargo:rustc-codegen-opts={}", codegen);
}
