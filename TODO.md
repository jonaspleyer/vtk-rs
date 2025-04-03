## Link Automatically with Cmake

1. Build (and link) `libvtkrs` with cmake
2. Build (but do not link) `vtkrs` with rustc
3. Link `vtkrs` library with `libvtkrs` and all Automatically obtained VTK libraries with Cmake

This should probably be done by creating a rustc-wrapper.
See https://doc.rust-lang.org/cargo/reference/config.html

```toml
# .cargo/config.toml
[build]
rustc = "rustc"
# This here should be crucial:
rustc-wrapper = "rustc_emit_and_cmake_linking"
```

We probably will have to write a wrapper which needs to be installed beforehand.
It is unclear if this is possible this way.

```toml
# Cargo.toml
[build-dependencies]
rustc_cmake_wrapper = { path="rustc_cmake_wrapper" }
```

Get inspiration by https://github.com/mozilla/sccache/.
