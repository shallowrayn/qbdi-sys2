use std::path::PathBuf;

fn get_lib_name() -> &'static str {
    // "QBDI_static"
    "QBDI"
}

fn get_additional_include_dirs() -> Vec<PathBuf> {
    vec![]
}

fn get_lib_dir() -> Option<PathBuf> {
    None
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib={}", get_lib_name());
    if let Some(lib_dir) = get_lib_dir() {
        println!(
            "cargo:rustc-link-search={}",
            lib_dir
                .to_str()
                .expect("Failed to convert library directory to string")
        );
    }

    let additional_include_dirs = get_additional_include_dirs();
    let clang_flags = additional_include_dirs
        .into_iter()
        .map(|d| {
            format!(
                "-I{}",
                d.to_str()
                    .expect("Failed to convert include directory to string")
            )
        })
        .collect::<Vec<_>>();
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(clang_flags)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
