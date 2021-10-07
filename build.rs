use std::env;
use std::path::Path;
use std::process::Command;
fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").expect("uhoh");
    let c_lib_path = format!("{}/CUDA/", manifest);
    let c_lib = Path::new(&c_lib_path);
    Command::new("meson")
        .arg("build")
        .current_dir(&c_lib)
        .output()
        .unwrap();
    Command::new("meson")
        .arg("compile")
        .current_dir(&c_lib.join("build"))
        .output()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", c_lib_path);
    println!("cargo:rustc-link-lib=dylib=cuda");
}
