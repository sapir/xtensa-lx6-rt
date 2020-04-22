use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    
    fs::copy(
            format!("bin/{}.a", "xtensa_vectors"),
            out.join("libxtensa_vectors.a"),
        ).unwrap();
        println!("cargo:rustc-link-lib=static=xtensa_vectors");

    // Put the linker script somewhere the linker can find it
    let mut link = File::create(out.join("link.x")).unwrap();
    link.write_all(include_bytes!("xtensa.in.x")).unwrap();
    link.write_all(b"\n").unwrap();
    link.write_all(include_bytes!("esp32.rom.ld")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=xtensa.in.x");
}
