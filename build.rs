use cmake::Config;

fn check_vtk_version() {
    let any = evaluate_feature(|_| ()).is_some();
}

fn evaluate_feature<F, G>(func: F) -> Option<G>
where
    F: Fn(&str) -> G,
{
    if cfg!(feature = "vtk9-4") {
        Some(func("vtk9-4"))
    } else if cfg!(feature = "vtk9-3") {
        Some(func("vtk9-3"))
    } else if cfg!(feature = "vtk9-2") {
        Some(func("vtk9-2"))
    } else if cfg!(feature = "vtk9-1") {
        Some(func("vtk9-1"))
    } else if cfg!(feature = "vtk9-0") {
        Some(func("vtk9-0"))
    } else if cfg!(feature = "vtk8-2") {
        Some(func("vtk8-2"))
    } else if cfg!(feature = "vtk8-1") {
        Some(func("vtk8-1"))
    } else if cfg!(feature = "vtk8-0") {
        Some(func("vtk8-0"))
    } else if cfg!(feature = "vtk7-1") {
        Some(func("vtk7-1"))
    } else if cfg!(feature = "vtk7-0") {
        Some(func("vtk7-0"))
    } else if cfg!(feature = "vtk6-3") {
        Some(func("vtk6-3"))
    } else if cfg!(feature = "vtk6-2") {
        Some(func("vtk6-2"))
    } else if cfg!(feature = "vtk6-1") {
        Some(func("vtk6-1"))
    } else if cfg!(feature = "vtk6-0") {
        Some(func("vtk6-0"))
    } else if cfg!(feature = "vtk5-10") {
        Some(func("vtk5-10"))
    } else if cfg!(feature = "vtk5-8") {
        Some(func("vtk5-8"))
    } else if cfg!(feature = "vtk5-6") {
        Some(func("vtk5-6"))
    } else if cfg!(feature = "vtk5-4") {
        Some(func("vtk5-4"))
    } else if cfg!(feature = "vtk5-2") {
        Some(func("vtk5-2"))
    } else if cfg!(feature = "vtk5-0") {
        Some(func("vtk5-0"))
    } else {
        None
    }
}

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

    let suffix = evaluate_feature(|version| {
        let chunks: Vec<_> = version[3..].split("-").collect();
        if chunks[1] == "0" {
            format!("-{}", chunks[0])
        } else {
            format!("-{}.{}", chunks[0], chunks[1])
        }
    })
    .unwrap_or("".to_string());
    for line in linker_args_raw.lines() {
        println!("cargo:rustc-link-lib=dylib={}{}", line, suffix);
    }
}
