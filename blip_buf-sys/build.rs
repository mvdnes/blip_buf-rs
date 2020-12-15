extern crate cc;

fn main() {
    cc::Build::new()
        .file("blip_buf.c")
        .compile("blip_buf");
}
