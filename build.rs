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

    let suffix = if cfg!(feature = "vtk9-4") {
        "9.4"
    } else if cfg!(feature = "vtk9-3") {
        "9.3"
    } else if cfg!(feature = "vtk9-2") {
        "9.2"
    } else if cfg!(feature = "vtk9-1") {
        "9.1"
    } else if cfg!(feature = "vtk9-0") {
        "9-0"
    } else if cfg!(feature = "vtk8-2") {
        "8-2"
    } else if cfg!(feature = "vtk8-1") {
        "8-1"
    } else if cfg!(feature = "vtk8-0") {
        "8-0"
    } else if cfg!(feature = "vtk7-1") {
        "7-1"
    } else if cfg!(feature = "vtk7-0") {
        "7-0"
    } else if cfg!(feature = "vtk6-3") {
        "6-3"
    } else if cfg!(feature = "vtk6-2") {
        "6-2"
    } else if cfg!(feature = "vtk6-1") {
        "6-1"
    } else if cfg!(feature = "vtk6-0") {
        "6-0"
    } else if cfg!(feature = "vtk5-10") {
        "5-10"
    } else if cfg!(feature = "vtk5-8") {
        "5-8"
    } else if cfg!(feature = "vtk5-6") {
        "5-6"
    } else if cfg!(feature = "vtk5-4") {
        "5-4"
    } else if cfg!(feature = "vtk5-2") {
        "5-2"
    } else if cfg!(feature = "vtk5-0") {
        "5-0"
    } else {
        ""
    };

    for line in linker_args_raw.lines() {
        println!("cargo:rustc-link-lib=dylib={}{}", line, suffix);
    }
}
