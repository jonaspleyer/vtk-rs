# Internals

1. [x] Generate xml data using WrapVTK
2. [x] Parse xml data
3. [x] Construct inheritance hierarchy for all classes in all modules
4. [x] Identify traits as non-inherited public methods
5. [ ] Define 1:1 conversion between `C++` and Rust types
    - Remove/Treat modifiers such as `const` or `unsigned`
    - Destructure Generics `std::array<float, 3>` <-> `[f32; 3]`
    - Treat positioning of pointers/references
6. [ ] Generate Code for ffi and traits
    - Generate `C++` code which can be wrapped in Rust
    - Generate `CMakeLists` file
    - Determine linker args
    - Generate Rust ffi glue and implementation code

In the future I will also have to decide how to publish the generated code.
I will probably provide crates with corresponding version numbers (i.e. `vtk-rs094`, `vtk-rs091`)
and also provide the tool `wrap-vtk` which can generate the bindings as a separate crate.
