use std::env;
use std::path::PathBuf;

fn get_lib_name() -> String {
    if let Ok(lib_name) = env::var("QBDI_LIB_NAME") {
        return lib_name;
    }

    // "QBDI_static"
    "QBDI".to_owned()
}

fn get_lib_dir() -> Option<PathBuf> {
    if let Ok(lib_dir) = env::var("QBDI_LIB_DIR") {
        return Some(PathBuf::from(lib_dir));
    }

    None
}

fn get_additional_include_dirs() -> Vec<PathBuf> {
    if let Ok(include_dir) = env::var("QBDI_INCLUDE_DIR") {
        return vec![PathBuf::from(include_dir)];
    }

    vec![]
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

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
