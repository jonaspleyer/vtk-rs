use cmake::Config;

fn determine_version_number(
    link_paths: &[std::path::PathBuf],
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if let Ok(v) = std::env::var("VTK_VERSION") {
        return Ok(Some(v));
    }

    if cfg!(unix) || cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        let re = regex::Regex::new("([.]*)libvtkCommonCore([0-9-]*).so")?;
        // Search in every provided link path
        for path in link_paths.iter() {
            // Gather candidates
            let search_path = path.join("libvtkCommonCore*.so").display().to_string();
            let candidates = glob::glob(&search_path)?;

            // Match against a regex
            for candidate in candidates.into_iter().filter_map(|x| x.ok()) {
                if let Some((_, [_, version])) = re
                    .captures_iter(&candidate.display().to_string())
                    .map(|x| x.extract())
                    .next()
                {
                    return Ok(Some(version.to_string()));
                }
            }
        }
    }

    Ok(None)
}

// Handle building of cmake project
fn build_cmake() {
    println!("cargo:rerun-if-changed=libvtkrs");
    let dst = Config::new("libvtkrs").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vtkrs");
}

// Collect all paths where to search for libraries
// Also emits flags to link to said paths
fn gather_link_paths() -> Vec<std::path::PathBuf> {
    let mut link_paths = Vec::<std::path::PathBuf>::new();
    if let Ok(v) = std::env::var("VTK_DIR") {
        println!("cargo:rustc-link-search={v}");
        link_paths.push(v.into());
    }

    if cfg!(unix) {
        println!("cargo:rustc-link-search=/usr/lib/");
        println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");
        link_paths.extend(["/usr/lib".into(), "/usr/lib/x86_64-linux-gnu/".into()]);
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=/usr/lib/");
        println!("cargo:rustc-link-lib=dylib=c++");
        link_paths.push("/usr/lib/".into());
    }

    link_paths
}

fn main() {
    // Exit early without doing anything if we are building for docsrs
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    build_cmake();

    let link_paths = gather_link_paths();

    let version = determine_version_number(&link_paths);

    let linker_args_raw = include_str!("linker-args.txt");

    let suffix = version.unwrap_or_default().unwrap_or_default();

    if let Ok(vtk_dir) = std::env::var("VTK_DIR") {
        println!("cargo:rustc-link-search={vtk_dir}");
    }

    for line in linker_args_raw.lines() {
        println!("cargo:rustc-link-lib=dylib={}{}", line, suffix);
    }
}
