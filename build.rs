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

fn determine_version_suffix(link_path: &std::path::Path) -> Result<Option<String>> {
    if let Ok(v) = std::env::var("VTK_VERSION") {
        return Ok(Some(v));
    }

    if cfg!(unix) || cfg!(target_os = "linux") || cfg!(target_os = "macos") {
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

    // if VTK_DIR is specified, do not try to determine anything else and return
    if let Ok(v) = std::env::var("VTK_DIR") {
        println!("cargo:rustc-link-search={v}");
        link_paths.push(v.into());
        return Ok(link_paths);
    }

    // Search in paths where homebrew might install packages
    if cfg!(unix) || cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        link_paths.extend([
            std::path::PathBuf::from("/usr/local/Cellar/vtk/"),
            std::path::PathBuf::from("/opt/homebrew/Cellar/vtk"),
        ]);
        link_paths.extend(glob::glob("/opt/homebrew/lib/*vtk*")?.filter_map(|x| x.ok()));
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
    let mut suffixes: Vec<(std::path::PathBuf, String)> = link_paths
        .into_iter()
        .flat_map(|x| {
            determine_version_suffix(&x)
                .ok()
                .and_then(|y| y.map(|y| (x, y)))
        })
        .collect();
    log!("{suffixes:?}");
    suffixes.sort_by_key(|x| x.1.len());
    log!("{suffixes:?}");

    let (link_path, version_suffix) = suffixes.into_iter().next().unwrap_or_default();
    println!("cargo:rustc-link-search={}", link_path.display());

    let linker_args_raw = include_str!("linker-args.txt");
    for line in linker_args_raw.lines() {
        println!("cargo:rustc-link-lib=dylib={}{}", line, version_suffix);
    }

    Ok(())
}
