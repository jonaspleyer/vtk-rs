use cmake::Config;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

const VERSION_REGEX: &str = "(lib)vtkCommonCore([-0-9._]*).(so|dylib|so|dll|a|lib)";
static WARN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

macro_rules! log(
    ($($ti:tt)*) => {
        if WARN.load(std::sync::atomic::Ordering::Relaxed) {
            print!("cargo::warning=");
        }
        println!($($ti)*);
    };
);

fn determine_version_suffix(link_paths: &[std::path::PathBuf]) -> Result<Option<String>> {
    if let Ok(v) = std::env::var("VTK_VERSION") {
        return Ok(Some(v));
    }

    if cfg!(unix) || cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        let re = regex::Regex::new(VERSION_REGEX)?;
        // Search in every provided link path
        for path in link_paths.iter() {
            // Gather candidates
            let search_path = path.join("*vtkCommonCore*").display().to_string();
            let candidates = glob::glob(&search_path)?;

            // Match against a regex
            for (n, candidate) in candidates
                .into_iter()
                .enumerate()
                .filter_map(|(n, x)| x.ok().map(|y| (n, y)))
            {
                log!("[{:2}] Candidate: {}", n + 1, candidate.display());
                if let Some((_, [_lib, version, _suffix])) = candidate
                    .file_name()
                    .and_then(|x| x.to_str())
                    .and_then(|x| re.captures_iter(x).map(|x| x.extract()).next())
                {
                    log!("Determined version suffix: \"{}\"", version);
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
            std::path::PathBuf::from("/usr/local/Cellar/vtk/").join("*"),
            std::path::PathBuf::from("/opt/homebrew/Cellar/vtk").join("*"),
            std::path::PathBuf::from("/opt/homebrew/lib").join("*vtk*"),
        ];
        let re = regex::Regex::new(VERSION_REGEX)?;

        for path in brew_paths {
            let search_path = path.display().to_string();
            log!("Search Path: {}", search_path);
            let candidates = glob::glob(&search_path)?;

            for c in candidates.into_iter().filter_map(|x| x.ok()) {
                log!("Candidate: {}", c.display());
                for (_, [version]) in re
                    .captures_iter(&c.display().to_string())
                    .map(|x| x.extract())
                {
                    if !version.is_empty() {
                        log!("Found version: {}", version);
                        link_paths.push(path.join(version));
                        println!("cargo:rustc-link-search={}", path.join(version).display());
                    }
                }
            }
        }
    }

    Ok(link_paths)
}

fn main() -> Result<()> {
    if let Ok(val) = std::env::var("VERBOSE") {
        if val == "1" {
            WARN.store(true, std::sync::atomic::Ordering::Relaxed);
            log!("Verbose Logging Enabled");
        }
    }

    // Exit early without doing anything if we are building for docsrs
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    build_cmake();

    let link_paths = if let Ok(vtk_dir) = std::env::var("VTK_DIR") {
        println!("cargo:rustc-link-search={vtk_dir}");
        vec![std::path::PathBuf::from(vtk_dir)]
    } else {
        log!("-- Automatically Determine VTK Lib Path");
        gather_link_paths()?
    };

    log!("-- Determine Version Suffix");
    let version_suffix = determine_version_suffix(&link_paths)
        .unwrap_or_default()
        .unwrap_or_default();

    let linker_args_raw = include_str!("linker-args.txt");
    for line in linker_args_raw.lines() {
        println!("cargo:rustc-link-lib=dylib={}{}", line, version_suffix);
    }

    Ok(())
}
