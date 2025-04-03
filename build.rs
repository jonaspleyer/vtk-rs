use cmake::Config;

fn main() {
    // Exit early without doing anything if we are building for docsrs
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-changed=libvtkrs");

    let dst = Config::new("libvtkrs").build();

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=/usr/lib/");
        println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");
    }

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vtkrs");

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    }

    let linker_args_raw = include_str!("linker-args.txt");

    macro_rules! get_suffix(
        () => {""};
        ($feature_name:literal -> $version_string:literal $($tokens:tt)*) => {
            if cfg!(feature = $feature_name) {
                return $version_string;
            }
        };
    );

    fn suffix() -> &'static str {
        get_suffix!(
            "vtk9-4" -> "9.4"
            "vtk9-3" -> "9.3"
            "vtk9-2" -> "9.2"
            "vtk9-1" -> "9.1"
            "vtk9-0" -> "9.0"
        );
        ""
    }
    let suffix = suffix();

    for line in linker_args_raw.lines() {
        println!("cargo:rustc-link-lib=dylib={}{}", line, suffix);
    }
}
