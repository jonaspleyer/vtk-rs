[![Crates.io Version](https://img.shields.io/crates/v/vtk_rs?style=flat-square)](https://crates.io/crates/vtk-rs)
[![Crates.io License](https://img.shields.io/crates/l/vtk_rs?style=flat-square)](https://github.com/jonaspleyer/vtk-rs/blob/main/LICENSE)
[![Docs](https://img.shields.io/docsrs/vtk-rs?style=flat-square)](https://docs.rs/vtk-rs)

# vtk-rs

Rust bindings for the [Visualization Toolkit (VTK)](https://vtk.org/).

## Testing

| | stable | beta | nightly | Build |
|---|---|---|---|---|
| `ubuntu-24.04` | [![stable-ubuntu-24_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_ubuntu-24_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_ubuntu-24_04.yml) |[![beta-ubuntu-24_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_beta_ubuntu-24_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_beta_ubuntu-24_04.yml) |[![nightly-ubuntu-24_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_nightly_ubuntu-24_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_nightly_ubuntu-24_04.yml) |`cargo build --features vtk9-1` |
| `ubuntu-22.04` | [![stable-ubuntu-22_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_ubuntu-22_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_ubuntu-22_04.yml) |[![beta-ubuntu-22_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_beta_ubuntu-22_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_beta_ubuntu-22_04.yml) |[![nightly-ubuntu-22_04](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_nightly_ubuntu-22_04.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_nightly_ubuntu-22_04.yml) |`cargo build --features vtk9-1` |
| `macos-13` | [![stable-macos-13](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_macos-13.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_macos-13.yml) |[![beta-macos-13](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_beta_macos-13.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_beta_macos-13.yml) |[![nightly-macos-13](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_nightly_macos-13.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_nightly_macos-13.yml) |`cargo build --features vtk9-4` |
| `macos-14` | [![stable-macos-14](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_stable_macos-14.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_stable_macos-14.yml) |[![beta-macos-14](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_beta_macos-14.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_beta_macos-14.yml) |[![nightly-macos-14](https://img.shields.io/github/actions/workflow/status/jonaspleyer/vtk-rs/test_nightly_macos-14.yml?style=flat-square&label=CI)](https://github.com/jonaspleyer/vtk-rs/actions/workflows/test_nightly_macos-14.yml) |`cargo build --features vtk9-4` |

## Dependencies

This package relies on a system install of `vtk`.
In some scenarios, it might be necessary to install additional dependencies.
Otherwise, compilation of the `cmake` part might fail linker errors.

| Distro | Packages |
| --- | --- |
| Archlinux | `pacman -S clang cmake vtk openmpi fast_float nlohmann-json gl2ps utf8cpp` |
| Ubuntu 22 & 24 | `sudo apt install libvtk9.1 libvtk9-dev` |

## Building
`vtk-rs` will try to determine the path for `vtk` automatically.
It is also possible to specify said path directly with the `VTK_DIR` environment variable.

## Internals
This crate builds on [`cmake`](https://docs.rs/cmake/latest/cmake/) and [`cxx`](https://cxx.rs/)
in order to generate the necessary code and bindings.
We manually write code for individual modules of `vtk` from which we generate appropriate bindings
with `cxx::bridge` using the provided CLI tool
[`cxxbridge`](https://crates.io/crates/cxxbridge-cmd).
However, we do not use `cxx` to compile the code but rather let `cmake` handle this task.

## Roadmap
1. Stabilize Build system
2. Automate system library detection and generate linker flags
3. Gradually implement functionality for examples. Start with 3D geometry.
- *SphereSource*
  https://examples.vtk.org/site/Cxx/GeometricObjects/SphereSource/
- *CylinderExample*
  https://examples.vtk.org/site/Cxx/GeometricObjects/CylinderExample/
