[![Crates.io Version](https://img.shields.io/crates/v/vtk_rs?style=flat-square)](https://crates.io/crates/vtk-rs)
[![Crates.io License](https://img.shields.io/crates/l/vtk_rs?style=flat-square)](https://github.com/jonaspleyer/vtk-rs/blob/main/LICENSE)
[![Docs](https://img.shields.io/docsrs/vtk-rs?style=flat-square)](https://docs.rs/vtk-rs)

# vtk-rs

Rust bindings for the [Visualization Toolkit (VTK)](https://vtk.org/).

## Scope
The goal of this project is to provide safe and thin bindings.
This means we are planning to support as much of the original functionality as possible, provided
that their use is reasonable from a rusty point of view.
This crate does in particular not aim at formulating higher-level interfaces such as
[pyvista](https://docs.pyvista.org/) although such functionality could be added in the future within
the scope of additional crates.

## ❗ Note ❗
This crate will be reworked using
[vtkWrap](https://docs.vtk.org/en/latest/advanced/WrappingTools.html) in order to automate much of
the process of generating the bindings.
In its current state, the crate will probably remain unusable for now.

## Testing

| | stable | Env Flags + Command |
|---|---|---|
| `ubuntu-24.04` | [![stable-ubuntu-24_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_ubuntu-24_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_ubuntu-24_04.yml) |`cargo test` |
| `ubuntu-22.04` | [![stable-ubuntu-22_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_ubuntu-22_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_ubuntu-22_04.yml) |`cargo test` |
| `macos-14` | [![stable-macos-14](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_macos-14.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_macos-14.yml) |`cargo test` |
| `macos-15` | [![stable-macos-15](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_macos-15.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_macos-15.yml) |`cargo test` |
| `macos-26` | [![stable-macos-26](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_macos-26.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_macos-26.yml) |`cargo test` |

## Dependencies

This package relies on a system install of `vtk`.
We currently only test versions `>=9.1`.
In some scenarios, it might be necessary to install additional dependencies.
Otherwise, compilation of the `cmake` part might fail with spurious linker errors.

| Distro | Packages |
| --- | --- |
| Archlinux | `pacman -S clang cmake vtk openmpi fast_float nlohmann-json gl2ps utf8cpp` |
| Ubuntu 22 & 24 | `apt install libvtk9.1 libvtk9-dev` |
| Macos 13 & 14 | `brew install vtk` |

## Building
`vtk-rs` will try to determine the path for `vtk` automatically.
It is possible to control the compilation process via environment flags.
| Flag | Effect |
| --- | --- |
| `VTK_DIR` | `cargo:rustc-link-search=$VTK_DIR` |
| `VTK_VERSION` | Add suffix to vtk libraries (i.e. `libvtkCommonCore-9.4`). |

## Internals
This crate builds on [`cmake`](https://docs.rs/cmake/latest/cmake/) and [`cxx`](https://cxx.rs/)
in order to generate the necessary code and bindings.
The bindings for `vtk` modules are written manually in `C++`.
From there, we generate appropriate bindings with `cxx::bridge` using the CLI tool
[`cxxbridge`](https://crates.io/crates/cxxbridge-cmd).
However, we do not use `cxx` to compile the code but rather let `cmake` handle this task.
To implement the desired class methods, we use Rust
[macros](https://doc.rust-lang.org/reference/macros-by-example.html).

## Contributing / Development Setup

This section is for contributors who want to regenerate bindings for a new VTK version.
If you only want to *use* the existing `vtk-rs-9.1` bindings, skip this section — a system VTK install and `cargo build` is sufficient.

### Prerequisites

Before following the steps below, ensure you have the following tools installed:

| Tool | Required | Purpose |
| --- | --- | --- |
| `cmake` (≥ 3.12) | Yes | Build VTK from source and build WrapVTK |
| `git` | Yes | Clone the VTK source repository |
| C++ compiler (`gcc` or `clang`) | Yes | Compile VTK and WrapVTK |
| Python dev headers | Yes | Required by VTK's Python wrapping layer, which WrapVTK depends on to generate XML |
| `libarchive` dev headers | Optional | Enables the `vtkCommonArchive` module; skipped gracefully if absent |
| ~10 GB free disk space | Yes | VTK source clone + build artifacts |

Install on Ubuntu 22 / 24:
```bash
# Required
sudo apt install cmake build-essential python3-dev
# Optional (for vtkCommonArchive)
sudo apt install libarchive-dev
```
Install on Arch Linux:
```bash
sudo pacman -S cmake gcc python libarchive
```
Install on macOS:
```bash
brew install cmake python libarchive
```

### 1. Run the setup script

`libvtk9-dev` (the system package) does **not** install the internal wrapping tool headers (e.g. `vtkParseAttributes.h`) that WrapVTK needs.
Use the provided `setup_vtk.sh` script to clone VTK from source, build it, initialise the `WrapVTK` submodule if needed, build WrapVTK against it, and verify the XML output — all in one step:

```bash
./setup_vtk.sh 9.2.0
```

The version argument is optional and defaults to `9.2.0`. This takes roughly 15–30 minutes depending on your machine.

The script will:
1. Wipe any existing `~/VTK` clone and `WrapVTK/build` directory
2. Clone VTK at the exact tag (e.g. `v9.2.0`) into `~/VTK`
3. Build VTK with static libs, Python wrapping enabled (required by WrapVTK), and the non-Common groups disabled (Rendering, Imaging, Qt, Web, Views, MPI — not needed for XML generation, and some contain version-specific compile bugs)
4. Initialise the `WrapVTK` git submodule automatically if not already done
5. Build WrapVTK against the fresh VTK build
6. Verify that XML files were generated under `WrapVTK/build/xml/`

### 2. Regenerate bindings with vtk-gen

```bash
cargo run -p vtk-gen -- \
  --opath vtk-rs-9.2 \
  --wrap-vtk WrapVTK
```

This regenerates all files in `vtk-rs-9.2/` from the WrapVTK XML output.
To target a different VTK version, run `./setup_vtk.sh <version>` first, then repeat this step with a matching output path (e.g. `--opath vtk-rs-9.3`).

## Roadmap
1. [x] Stabilize Build system
2. [x] Automate system library detection and generate linker flags
3. [ ] Gradually implement functionality for examples. Start with 3D geometry.
- *SphereSource*
  https://examples.vtk.org/site/Cxx/GeometricObjects/SphereSource/
- *CylinderExample*
  https://examples.vtk.org/site/Cxx/GeometricObjects/CylinderExample/
