# Internals
In order to correctly determine which methods and types to expose, `vtk-wrap` converts the
information obtained by `WrapVTK` between multiple layers until the desired code can be generated.

1. [x] Generate xml data using WrapVTK
2. [x] Parse xml data into custom Rust structs (module `parse_wrap_vtk_xml.rs`)
3. [x] Parse C++ types (module `parse_cpp.rs`)
4. [ ] Gather information on exposable methods, defined types, modules, features etc. (modules
   `inheritance_hierarchy.rs`)
5. [ ] Convert to rust-specific information (module `intermediate.rs`)
6. [ ] Generate ffi code (module `code_gen.rs`)
7. [ ] Generate C++ code
    - Generate `CmakeLists` file
    - Determine linker args
8. [ ] Generate Tests

In the future I will also have to decide how to publish the generated code.
I will probably provide crates with corresponding version numbers (i.e. `vtk-rs094`, `vtk-rs091`)
and also provide the tool `wrap-vtk` which can generate the bindings as a separate crate.
