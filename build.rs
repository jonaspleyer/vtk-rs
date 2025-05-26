use cmake::Config;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn determine_version_suffix(link_paths: &[std::path::PathBuf]) -> Result<Option<String>> {
    if let Ok(v) = std::env::var("VTK_VERSION") {
        return Ok(Some(v));
    }

    if cfg!(unix) || cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        let re = regex::Regex::new("libvtkCommonCore([-0-9.]*).so")?;
        // Search in every provided link path
        for path in link_paths.iter() {
            // Gather candidates
            let search_path = path.join("libvtkCommonCore*.so").display().to_string();
            let candidates = glob::glob(&search_path)?;

            // Match against a regex
            for candidate in candidates.into_iter().filter_map(|x| x.ok()) {
                if let Some((_, [version])) = re
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
fn gather_link_paths() -> Result<Vec<std::path::PathBuf>> {
    let mut link_paths = Vec::<std::path::PathBuf>::new();
    if let Ok(v) = std::env::var("VTK_DIR") {
        println!("cargo:rustc-link-search={v}");
        link_paths.push(v.into());
    }

    if cfg!(unix) {
        let paths: [std::path::PathBuf; 3] = [
            "/usr/lib".into(),
            "/usr/lib/x86_64-linux-gnu/".into(),
            "/usr/local/lib/".into(),
        ];
        for p in paths.iter() {
            println!("cargo:rustc-link-search={}", p.display());
        }
        link_paths.extend(paths);
    }

    // Link with c++ standard library
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    }

    // Search in paths where homebrew might install packages
    if cfg!(target_os = "macos") || cfg!(unix) {
        let brew_paths = [
            std::path::PathBuf::from("/usr/local/Cellar/vtk/"),
            std::path::PathBuf::from("/opt/homebrew/Cellar/vtk"),
            std::path::PathBuf::from("/opt/homebrew/lib"),
        ];
        let re = regex::Regex::new("([-0-9.]*)")?;

        for path in brew_paths {
            let candidates = glob::glob(&path.join("*").display().to_string())?;

            for c in candidates.into_iter().filter_map(|x| x.ok()) {
                for (_, [version]) in re
                    .captures_iter(&c.display().to_string())
                    .map(|x| x.extract())
                {
                    println!("cargo:rustc-link-search={}", path.join(version).display());
                }
            }
        }
    }

    Ok(link_paths)
}

fn main() -> Result<()> {
    // Exit early without doing anything if we are building for docsrs
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    build_cmake();

    let link_paths = gather_link_paths()?;

    let version_suffix = determine_version_suffix(&link_paths)
        .unwrap_or_default()
        .unwrap_or_default();

    if let Ok(vtk_dir) = std::env::var("VTK_DIR") {
        println!("cargo:rustc-link-search={vtk_dir}");
    }

    let linker_args_raw = include_str!("linker-args.txt");
    for line in linker_args_raw.lines() {
        println!("cargo:rustc-link-lib=dylib={}{}", line, version_suffix);
    }

    Ok(())
}
