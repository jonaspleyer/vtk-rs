type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// Regex to search for version of vtk library
pub const VERSION_REGEX: &str = "(lib)vtkCommonCore([-0-9._]*).(so|dylib|so|dll|a|lib)";
/// AtomicBool to store flag if logging is enabled
pub static WARN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Used to log to the console during build steps
#[macro_export]
macro_rules! log(
    ($($ti:tt)*) => {
        if WARN.load(std::sync::atomic::Ordering::Relaxed) {
            print!("cargo::warning=");
        }
        println!($($ti)*);
    };
);

/// Bundles information about the current vtk install
#[derive(Clone)]
pub struct VTKVersionInfo {
    pub version: Option<String>,
    pub dir: Option<String>,
    pub version_suffix: String,
}

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

/// Try to find information about the VTK installation.
///
/// See [VTKVersionInfo]
pub fn find_vtk_info() -> Result<VTKVersionInfo> {
    let vtk_version = std::env::var("VTK_VERSION").ok();
    let vtk_dir = std::env::var("VTK_DIR").ok();

    let version_suffix: String = match (&vtk_version, &vtk_dir) {
        (Some(v), Some(d)) => {
            if !std::path::PathBuf::from(&d).is_dir() {
                return Err(format!("Given Path {d} does not exist in filesystem").into());
            }
            println!("cargo:rustc-link-search={d}");
            v.clone()
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
                        if suffix.unwrap_or_default().contains(v) {
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
            v.clone()
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

    Ok(VTKVersionInfo {
        version: vtk_version,
        dir: vtk_dir,
        version_suffix,
    })
}
