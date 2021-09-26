use std::path::PathBuf;
use std::process::Command;
extern crate cc;
fn main() {
    let mut cuda_lib = PathBuf::from("./CUDA");
    Command::new("meson")
        .arg("build")
        .current_dir(&cuda_lib)
        .output()
        .unwrap();
    cuda_lib.push("build");
    Command::new("meson")
        .arg("compile")
        .current_dir(&cuda_lib)
        .output()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", (*cuda_lib).display());
    println!("cargo:rustc-link-lib=dylib=cuda");
}
