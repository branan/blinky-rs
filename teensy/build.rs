extern crate rustc_serialize;
use rustc_serialize::base64;
use rustc_serialize::base64::ToBase64;

use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let arg = format!("OUTDIR={}", outdir);

    let output = Command::new("make").arg("-C").arg("c_bits").arg(arg).output().unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if ! output.status.success() {
        panic!("Didn't successfully make!: {}", stderr);
    }

    let canonical_path = Path::new("layout.ld").canonicalize().unwrap();
    let layout_file = canonical_path.to_str().unwrap();
    let codegen_opts = vec!["link-arg=-nostartfiles".to_owned(),
                            format!("link-arg=-T{}", layout_file)];
    let codegen_opts = codegen_opts.join(",").as_bytes().to_base64(base64::URL_SAFE);
    
    println!("cargo:rustc-link-search=native={}", outdir);
    println!("cargo:rustc-link-lib=static=teensyrs");
    println!("cargo:codegen={}", codegen_opts);
}
