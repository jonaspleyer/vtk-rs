use cmake::Config;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

use vtk_rs_link::{log, WARN};

// Handle building of cmake project
fn build_cmake() {
    println!("cargo:rerun-if-changed=libvtkrs");
    let mut config = Config::new("libvtkrs");

    if std::env::var("CARGO_FEATURE_V094").is_ok_and(|x| x == "1") {
        config.define("VTK094", "1");
    }

    let dst = config.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vtkrs");
}

fn main() -> Result<()> {
    // Exit early without doing anything if we are building for docsrs
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    if let Ok(val) = std::env::var("VERBOSE") {
        if val == "1" || val.to_lowercase() == "true" {
            WARN.store(true, std::sync::atomic::Ordering::Relaxed);
            log!("-- Verbose Logging Enabled");
        }
    }

    // Build cpp project
    build_cmake();

    let linker_args_raw = include_str!("linker-args.txt");
    let modules = linker_args_raw.lines();
    vtk_rs_link::link_cmake_project(modules)?;

    Ok(())
}
