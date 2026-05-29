## [0.3.0] - 2026-05-29
This change will introduce the new feature of automatically creating the hierarchy. Additionally, tests were added for all the core functionalities of `vtk-gen` (not complete for possible edge cases, focused on straight forward functionality assessment). Most other changes are a result of getting everything to build without errors (e.g. handling rust keywords, handling different cpp pointers, ...).

Detailed summary:

### Added
- `vtkFiltersSources` module is now generated and linked, adding `SphereSource` and 20+ other source classes.
- Re-export all constructable VTK classes at the crate root with the `vtk` prefix stripped (e.g. `vtk_rs::SphereSource`, `vtk_rs::NamedColors`).
- `pub mod prelude` in the generated `lib.rs` that re-exports all module contents via glob, enabling `use vtk_rs::prelude::*` for ergonomic trait method access without explicit trait imports.
- `sphere_source` example in `examples/` (workspace root), runnable via `cargo run --example sphere_source`.
- `has_ancestor(class_name, target) -> bool` on `ClassHierarchy` -> iterative DFS to walk the full ancestor chain (not just direct parents). Used to correctly detect `vtkObjectBase` ancestry across deep inheritance hierarchies.
- `IRStruct::has_vtk_object_base_ancestor: bool` field, computed via `has_ancestor` rather than checking direct parents only, fixing `is_constructable()` for deeply inherited classes.
- `IRMethod::vtk_name: String` field (PascalCase VTK method name) and `short_name()` helper for generating clean Rust method names.
- `c_signed_char` IR type to distinguish VTK's explicitly-typed `signed char` (used in typed data arrays such as `vtkSignedCharArray`) from plain `char` (used for C strings). `CppType::PlainChar` added to the C++ parser for the same reason.
- Overload deduplication via `.scan()` in `IRModule::new()`: C does not support overloading, so only the first VTK overload seen for each binding name is kept.
- `StarStarConst` (`**const`) and `StarStarStar` (`***`) pointer variants added to the `Pointer` enum in `parse_wrap_vtk_xml`.
- `pointer: Option<Pointer>` field added to `Parameter` in `parse_wrap_vtk_xml` so parameter pointer qualifiers are no longer silently dropped.
- C-style array parameter filter in `get_exposable_methods`: methods whose signature contains `[` are skipped because array extents are not encoded in the WrapVTK XML.
- Comprehensive test suites: `#[cfg(test)] mod inheritance_tests` (5 tests), `mod gen_rust_tests` (20 tests), `mod gen_cpp_tests` (8 tests), `mod tests` in `intermediate_representation` (5 tests). Total: 52 vtk-gen unit tests.

### Changed
- `is_constructable()` now requires `has_vtk_object_base_ancestor` (full ancestor chain) instead of checking only direct parents for `vtkObjectBase`.
- `method_to_cpp` / `method_to_cpp_header` emit `extern "C"` bindings using `method.vtk_name` for the C++ call site and `method.name` for the symbol name.
- The glob processed by vtk-gen now includes `vtkFiltersSources` in addition to `vtkCommon*`.
- `write_build_rs` in `vtk-gen/src/main.rs` no longer hardcodes `vtktoken` in the link list (VTK 9.1 system packages do not ship `libvtktoken`).

### Fixed
- **`vtkNew<T>` ABI mismatch in generated C++ wrappers**: `vtkNew<T>` has a non-trivial destructor, so passing or returning it by value in `extern "C"` functions violates the x86-64 SysV ABI and causes the wrapped VTK object to be destroyed on every method call. Constructor, destructor, get-ptr, and all method wrappers now use raw `T*` (`T::New()` / `sself->Delete()` / `return sself`) instead of `vtkNew<T>`.
- **`std::string` return types** (`IRType::String`): bridging `std::string` as `const char*` is illegal (dangling pointer). These methods are now skipped on both the Rust and C++ sides.
- **`const char*` vs `const char* const*`**: `StarStarConst` and `StarStar` now always bail instead of incorrectly reducing to a single pointer.
- **Mutable `char**` output parameters**: `StarStar` always bails -> VTK_FILEPATH uses `pointer="*"` in the actual XML, never `pointer="**"`. The previous special-case for `Const(SignedChar)**` was unreachable dead code and has been removed.
- **Mutable `char*` vs `signed char*`**: `Pointer(c_char)` (without `Const`) is now rejected on both Rust and C++ sides — VTK typed-data-array methods use `signed char*`, which is not implicitly convertible from `char*` in C++.
- **`signed char*` data array methods**: Separated `CppType::PlainChar` (`"char"`) from `CppType::SignedChar` (`"signed char"`) so typed-data-array parameters (`const signed char*`) generate the correct C++ type and are rejected at the Rust FFI boundary (not safely bridgeable without element-count information).
- **`Path` types by value/reference** (e.g. `const vtkStdString&`, `vtkColor3ub`): rejected in `ir_type_is_supported`; VTK object types are only bridgeable as opaque pointers.
- **Heap collection types** (`Vec`, `LinkedList`, `Map`): rejected in `ir_type_is_supported` — these types cannot cross the `extern "C"` boundary safely.
- **Cross-module supertrait bounds**: removed from generated trait definitions. Generating `trait VtkFoo: VtkBar` across module boundaries requires the concrete struct to implement all ancestor traits, which the generator does not yet support.
- **`vtktoken` linker error**: removed from the hardcoded link list in `write_build_rs`; `libvtktoken` is a VTK 9.2+ library not present in VTK 9.1 system packages.
- **`c_longlong` Rust type mapping**: was incorrectly emitting `core::ffi::c_uchar` (copy-paste error); now correctly emits `core::ffi::c_longlong`.
- **Generated `test_vtkXxx_create_drop` was broken**: after the `vtkNew<T>` → `T*` switch, `get_ptr` returns the pointer itself so the post-drop null assertion was wrong and the test accessed freed memory (UB). Simplified to verify creation gives a non-null pointer and `drop` does not panic.
- **Panic in `get_exposable_methods`**: indexing `self.classes[parent]` would panic if a parent class named in an XML `<inheritance>` entry had no corresponding XML file scanned. Changed to a silent skip via `.filter_map(|n| self.classes.get(&n))`.
- **`new()` in generated bindings**: `Self(unsafe { &mut *constructor() })` created a spurious `&mut c_void` reference before coercing back to `*mut c_void`. Simplified to `Self(unsafe { constructor() })`.

## [0.2.0] - 2025-06-03

## [0.1.3] - 2025-05-25

## [0.1.2] - 2025-05-24

## [0.1.1] - 2025-04-01