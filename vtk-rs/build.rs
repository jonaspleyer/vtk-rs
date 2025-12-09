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

/// If VTK_VERSION && !VTK_DIR:
///      => set version_suffix
///      => Search for matching VTK Version in possible link paths
///      => Hopefully find something and emit linker vars
/// If VTK_VERSION && VTK_DIR::
///      => Simply emit this linker combination
/// If !VTK_VERSION && VTK_DIR:
///      => Search for matching VTK Version in given path
/// If !VTK_VERSION && !VTK_DIR:
///      => Search in possible link paths for any version of vtk
///         by inferring version from (lib)vtkCommonCore{version}.(so|lib|...)
///      => Hopefully find something and emit linker vars
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

    let vtk_rs_link::VTKVersionInfo {
        version: _,
        dir: _,
        version_suffix,
    } = vtk_rs_link::find_vtk_info()?;

    // Build cpp project
    build_cmake();

    let linker_args_raw = include_str!("linker-args.txt");
    for line in linker_args_raw.lines() {
        println!("cargo:rustc-link-lib=dylib={}{}", line, version_suffix);
    }

    // Link with c++ standard library
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    }

    Ok(())
}
