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

fn find_version_suffix(link_path: &std::path::Path) -> Result<Option<String>> {
    let re = regex::Regex::new(VERSION_REGEX)?;
    // Search in every provided link path
    // Gather candidates
    let search_path = link_path.join("*vtkCommonCore*").display().to_string();
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

    Ok(None)
}

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

// Collect all paths of directories where to search for libraries
fn gather_link_paths() -> Result<impl IntoIterator<Item = std::path::PathBuf>> {
    let mut link_paths = Vec::<std::path::PathBuf>::new();

    if cfg!(unix) {
        link_paths.extend([
            "/usr/lib/x86_64-linux-gnu/".into(),
            "/usr/local/lib/".into(),
            "/usr/lib".into(),
        ]);
    }

    // Search in paths where homebrew might install packages
    if cfg!(unix) || cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        link_paths.extend([
            "/usr/local/Cellar/vtk/".into(),
            "/opt/homebrew/Cellar/vtk".into(),
            "/opt/homebrew/lib/".into(),
        ]);
        link_paths.extend(glob::glob("/opt/homebrew/lib/*vtk*")?.filter_map(|x| x.ok()));
    }

    log!("Gather Link Paths");
    log!("Before Filtering");
    for p in link_paths.iter() {
        log!("{}", p.display());
    }

    let link_paths: Vec<_> = link_paths.into_iter().filter(|x| x.is_dir()).collect();

    log!("After Filtering");
    for p in link_paths.iter() {
        log!("{}", p.display());
    }
    Ok(link_paths)
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

    let vtk_version = std::env::var("VTK_VERSION").ok();
    let vtk_dir = std::env::var("VTK_DIR").ok();

    let version_suffix = match (vtk_version, vtk_dir) {
        (Some(v), Some(d)) => {
            if !std::path::PathBuf::from(&d).is_dir() {
                return Err(format!("Given Path {d} does not exist in filesystem").into());
            }
            println!("cargo:rustc-link-search={d}");
            v
        }
        (None, Some(d)) => {
            println!("cargo:rustc-link-search={d}");
            find_version_suffix(&std::path::PathBuf::from(d))?.unwrap_or_default()
        }
        (Some(v), None) => {
            // Search for matching VTK Verison in given path
            let link_paths = gather_link_paths()?;
            let candidates: Vec<_> = link_paths
                .into_iter()
                .filter_map(|link_path| {
                    if let Ok(suffix) = find_version_suffix(&link_path) {
                        log!("Candidate: {:?}", suffix);
                        if suffix.unwrap_or_default().contains(&v) {
                            log!("Matched in {}", link_path.display());
                            Some(link_path)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            if let Some(p) = candidates.into_iter().next() {
                println!("cargo:rustc-link-search={}", p.display());
            } else {
                panic!("Could not find suitable installation directory.");
            }
            v
        }
        (None, None) => {
            let link_paths = gather_link_paths()?;
            let mut versions: Vec<(std::path::PathBuf, String)> = link_paths
                .into_iter()
                .filter_map(|link_path| {
                    find_version_suffix(&link_path).ok().map(|x| (link_path, x))
                })
                .filter_map(|(p, s)| s.map(|x| (p, x)))
                .collect();
            versions.sort_by_key(|x| x.1.len());
            for (p, v) in versions.iter() {
                log!("path: {} version: {}", p.display(), v);
            }
            if let Some((path, version)) = versions.into_iter().next() {
                println!("cargo:rustc-link-search={}", path.display());
                version
            } else {
                panic!("Could not find suitable installation directory.");
            }
        }
    };

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
