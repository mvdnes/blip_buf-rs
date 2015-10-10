extern crate gcc;

use std::env;

fn main() {
    gcc::compile_library("libblip_buf.a", &["blip_buf.c"]);
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
