use std::env;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
fn get_windows_path() -> PathBuf {
    #[cfg(target_pointer_width = "64")]
    let program_files = PathBuf::from(r"C:\Program Files");
    #[cfg(target_pointer_width = "32")]
    let program_files = PathBuf::from(r"C:\Program Files (x86)");
    for entry in std::fs::read_dir(program_files).expect("Failed to read Program Files directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name() {
                if dir_name.to_string_lossy().starts_with("QBDI") {
                    return path;
                }
            }
        }
    }
    panic!("Failed to find QBDI installation directory")
}

fn get_lib_name() -> String {
    println!("cargo::rerun-if-env-changed=QBDI_LIB_NAME");
    if let Ok(lib_name) = env::var("QBDI_LIB_NAME") {
        return lib_name;
    }

    // Avoid needing to copy libQBDI.dll to the target directory
    // TODO: Is this a sensible default?
    if cfg!(target_os = "windows") {
        "QBDI_static".to_owned()
    } else {
        "QBDI".to_owned()
    }
}

fn get_lib_dir() -> Option<PathBuf> {
    println!("cargo::rerun-if-env-changed=QBDI_LIB_DIR");
    if let Ok(lib_dir) = env::var("QBDI_LIB_DIR") {
        return Some(PathBuf::from(lib_dir));
    }

    #[cfg(target_os = "windows")]
    {
        let installation_path = get_windows_path();
        let lib_dir = installation_path.join("lib");
        if lib_dir.is_dir() {
            return Some(lib_dir);
        }
    }

    None
}

fn get_additional_include_dirs() -> Vec<PathBuf> {
    println!("cargo::rerun-if-env-changed=QBDI_INCLUDE_DIR");
    if let Ok(include_dir) = env::var("QBDI_INCLUDE_DIR") {
        return vec![PathBuf::from(include_dir)];
    }

    #[cfg(target_os = "windows")]
    {
        let installation_path = get_windows_path();
        let include_dir = installation_path.join("include");
        if include_dir.is_dir() {
            return vec![include_dir];
        }
    }

    vec![]
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-link-lib={}", get_lib_name());
    if let Some(lib_dir) = get_lib_dir() {
        println!(
            "cargo::rustc-link-search={}",
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
