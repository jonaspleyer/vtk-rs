use cmake::Config;
use vtk_rs_link::{log, Result, WARN};
fn build_cmake() {
    println!("cargo:rerun-if-changed=libvtkrs");
    let mut config = Config::new("libvtkrs");
    let dst = config.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vtkrs");
}
fn main() -> Result<()> {
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }
    if let Ok(val) = std::env::var("VERBOSE") {
        if val == "1" || val.to_lowercase() == "true" {
            WARN.store(true, std::sync::atomic::Ordering::Relaxed);
            log!("-- Verbose Logging Enabled");
        }
    }
    build_cmake();
    let modules = vec![
        "vtksys",
        "vtktoken",
        "vtkCommonArchive",
        "vtkCommonColor",
        "vtkCommonComputationalGeometry",
        "vtkCommonCore",
        "vtkCommonDataModel",
        "vtkCommonExecutionModel",
        "vtkCommonMath",
        "vtkCommonMisc",
        "vtkCommonPython",
        "vtkCommonSystem",
        "vtkCommonTransforms",
    ];
    vtk_rs_link::link_cmake_project(modules)?;
    Ok(())
}
