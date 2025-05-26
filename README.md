[![Crates.io Version](https://img.shields.io/crates/v/vtk_rs?style=flat-square)](https://crates.io/crates/vtk-rs)
[![Crates.io License](https://img.shields.io/crates/l/vtk_rs?style=flat-square)](https://github.com/jonaspleyer/vtk-rs/blob/main/LICENSE)
[![Docs](https://img.shields.io/docsrs/vtk-rs?style=flat-square)](https://docs.rs/vtk-rs)

# vtk-rs

Rust bindings for the [Visualization Toolkit (VTK)](https://vtk.org/).

## Testing

| | stable | beta | nightly | Build |
|---|---|---|---|---|
| `ubuntu-24.04` | [![stable-ubuntu-24_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_ubuntu-24_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_ubuntu-24_04.yml) |[![beta-ubuntu-24_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_beta_ubuntu-24_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_beta_ubuntu-24_04.yml) |[![nightly-ubuntu-24_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_nightly_ubuntu-24_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_nightly_ubuntu-24_04.yml) |`VTK_VERSION=-9.1 cargo build` |
| `ubuntu-22.04` | [![stable-ubuntu-22_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_ubuntu-22_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_ubuntu-22_04.yml) |[![beta-ubuntu-22_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_beta_ubuntu-22_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_beta_ubuntu-22_04.yml) |[![nightly-ubuntu-22_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_nightly_ubuntu-22_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_nightly_ubuntu-22_04.yml) |`VTK_VERSION=-9.1 cargo build` |
| `macos-13` | [![stable-macos-13](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_macos-13.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_macos-13.yml) |[![beta-macos-13](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_beta_macos-13.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_beta_macos-13.yml) |[![nightly-macos-13](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_nightly_macos-13.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_nightly_macos-13.yml) |`VTK_VERSION=-9.4 cargo build` |
| `macos-14` | [![stable-macos-14](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_macos-14.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_macos-14.yml) |[![beta-macos-14](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_beta_macos-14.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_beta_macos-14.yml) |[![nightly-macos-14](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_nightly_macos-14.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_nightly_macos-14.yml) |`VTK_VERSION=-9.4 cargo build` |

## Dependencies

This package relies on a system install of `vtk`.
In some scenarios, it might be necessary to install additional dependencies.
Otherwise, compilation of the `cmake` part might fail linker errors.

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

## Roadmap
1. Stabilize Build system
2. Automate system library detection and generate linker flags
3. Gradually implement functionality for examples. Start with 3D geometry.
- *SphereSource*
  https://examples.vtk.org/site/Cxx/GeometricObjects/SphereSource/
- *CylinderExample*
  https://examples.vtk.org/site/Cxx/GeometricObjects/CylinderExample/
