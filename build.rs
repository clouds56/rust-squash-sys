extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper");
    println!("cargo:rerun-if-changed=squash/");
    fn get_deps(libname: &str) -> (Vec<String>, Vec<String>) {
        let library = match pkg_config::probe_library(libname) {
            Ok(library) => library,
            _ => {
                let prefix = cmake::Config::new("squash").build();
                env::set_var("PKG_CONFIG_PATH", prefix.join("lib").join("pkgconfig"));
                pkg_config::probe_library(libname).unwrap()
            }
        };
        let clang_args = library.include_paths.iter().map(|path| format!("-I{}", path.display())).chain(
            library.link_paths.iter().map(|path| format!("-L{}", path.display()))).collect();
        (clang_args, library.libs)
    }
    let (clang_args, libs) = get_deps("squash-0.8");

    let mut bindings = bindgen::Builder::default()
        .clang_args(clang_args)
        .header("wrapper.h")

        .bitfield_enum("SquashCodecInfo")
        .bitfield_enum("SquashLicense")
        .prepend_enum_name(false)

        .opaque_type("SquashObject")

        .blacklist_type("va_list")
        .blacklist_type(".*_$")
        .blacklist_type("FILE")
        // Blacklist_type blacklists functions too. See https://github.com/rust-lang-nursery/rust-bindgen/issues/1142
        .blacklist_type("^squash.*vw?$")
        .blacklist_type(".*vw?printf.*")

        .generate_comments(true)

        .rust_target(if cfg!(feature = "nightly") {
            bindgen::RustTarget::Nightly
        } else {
            bindgen::RustTarget::Stable_1_19
        })
        .whitelist_function("squash.*")
        .whitelist_type("Squash.*");
    for lib in libs {
        bindings = bindings.link(lib);
    }

    let bindings = bindings
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
