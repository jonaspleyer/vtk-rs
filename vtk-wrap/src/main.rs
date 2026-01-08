use anyhow::Result;

mod code_gen_cpp;
mod code_gen_rust;
mod inheritance_hierarchy;
mod intermediate_representation;
mod parse_cpp;
mod parse_wrap_vtk_xml;

use code_gen_cpp::*;
use code_gen_rust::*;
use inheritance_hierarchy::*;
use intermediate_representation::*;
use parse_wrap_vtk_xml::*;

fn create_folders(path: impl Into<std::path::PathBuf>) -> Result<()> {
    use std::fs::create_dir_all;
    let path: std::path::PathBuf = path.into();
    create_dir_all(path.join("libvtkrs/include"))?;
    create_dir_all(path.join("libvtkrs/src"))?;
    create_dir_all(path.join("src"))?;
    Ok(())
}

fn create_cmake_lists_txt(
    modules: &[&IRModule],
    path: impl Into<std::path::PathBuf>,
) -> Result<()> {
    // Change this into something possibly user-definable
    let cmake_version = "3.12";
    let project_name = "vtkrs";

    use std::io::Write;
    let path = path.into();
    let mut ofile = std::fs::File::create(path.join("CMakeLists.txt"))?;
    writeln!(
        &mut ofile,
        "cmake_minimum_required(VERSION {})",
        cmake_version
    )?;
    writeln!(&mut ofile)?;
    writeln!(&mut ofile, "project({})", project_name)?;
    writeln!(&mut ofile)?;

    // Find packages
    writeln!(&mut ofile, "find_package(VTK COMPONENTS")?;
    for m in modules {
        if let Some(vtkmodule_name) = m.vtk_module_name() {
            writeln!(&mut ofile, "    {}", vtkmodule_name)?;
        }
    }
    writeln!(&mut ofile, ")")?;

    writeln!(
        &mut ofile,
        r#"
if (NOT VTK_FOUND)
    message(FATAL_ERROR "Vtk: Unable to find the VTK build folder.")
endif()

# Prevent a "command line is too long" failure in Windows.
set(CMAKE_NINJA_FORCE_RESPONSE_FILE "ON" CACHE BOOL "Force Ninja to use response files.")
"#
    )?;

    // Add libraries
    writeln!(&mut ofile, "add_library({} STATIC", project_name)?;
    for m in modules {
        writeln!(
            &mut ofile,
            "    ${{PROJECT_SOURCE_DIR}}/include/{}.h",
            m.name_snake_case()
        )?;
    }
    writeln!(&mut ofile, ")")?;

    writeln!(
        &mut ofile,
        r#"
if (VTK094)
    target_compile_definitions(vtkrs PUBLIC -DVTK094=1)
endif()

include_directories(${{PROJECT_SOURCE_DIR}}/include/)"#
    )?;

    // Target sources
    writeln!(&mut ofile, "target_sources(vtkrs")?;
    writeln!(&mut ofile, "     PRIVATE")?;
    for m in modules {
        writeln!(
            &mut ofile,
            "        ${{PROJECT_SOURCE_DIR}}/src/{}.cpp",
            m.name_snake_case()
        )?;
    }
    writeln!(&mut ofile, ")")?;

    writeln!(
        &mut ofile,
        r#"
set_target_properties(vtkrs PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(vtkrs PRIVATE ${{VTK_LIBRARIES}})

install(TARGETS vtkrs DESTINATION .)

vtk_module_autoinit(
  TARGETS vtkrs
  MODULES ${{VTK_LIBRARIES}}
)"#
    )?;
    Ok(())
}

/// Generate C++ Code from IR Modules
fn write_cpp_module(
    class_hierarchy: &ClassHierarchy,
    module: &IRModule,
    writer: &mut impl std::io::Write,
) -> Result<()> {
    module.to_cpp_src(writer)?;
    Ok(())
}

fn write_cpp_header(
    class_hierarchy: &ClassHierarchy,
    module: &IRModule,
    writer: &mut impl std::io::Write,
) -> Result<()> {
    module.to_cpp_header(writer)?;
    Ok(())
}

/// Generate Rust code from IR Modules
fn write_rust_module(
    class_hierarchy: &ClassHierarchy,
    module: &IRModule,
    writer: &mut impl std::io::Write,
) -> Result<()> {
    let rust_module = quote::quote!(#module);
    let tokenstream = quote::quote!(#rust_module);
    let file: Result<syn::File, _> = syn::parse_file(&tokenstream.to_string());
    match file {
        Ok(res) => write!(writer, "{}", prettyplease::unparse(&res))?,
        Err(e) => {
            log::error!("Formatting failed: \"{e}\" Returning unformatted Code");
            write!(writer, "{}", tokenstream)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Obtain all modules
    let modules = get_modules("WrapVTK/build/xml/vtkCommon*")?;

    let class_hierarchy = ClassHierarchy::new(&modules)?;

    // Convert to intermediate representation from which C++ and Rust code will be generated
    let ir_modules = modules
        .into_iter()
        .map(|m| IRModule::new(m, &class_hierarchy))
        .collect::<Result<Vec<_>>>()?;

    // For testing purposes; filter for just one module
    let module = ir_modules
        .into_iter()
        .find(|x| x.name.contains("vtkCommonColor"))
        .unwrap();

    // Build directory where results will be generated into
    create_folders("test")?;
    create_cmake_lists_txt(&[&module], "test/libvtkrs")?;

    let mut cpp_file = std::fs::File::create(format!(
        "test/libvtkrs/src/{}.cpp",
        &module.name_snake_case()
    ))?;
    write_cpp_module(&class_hierarchy, &module, &mut cpp_file)?;
    let mut header_file = std::fs::File::create(format!(
        "test/libvtkrs/include/{}.h",
        &module.name_snake_case()
    ))?;
    write_cpp_header(&class_hierarchy, &module, &mut header_file)?;

    let mut rust_file = std::fs::File::create(format!("test/src/{}.rs", module.name))?;
    write_rust_module(&class_hierarchy, &module, &mut rust_file)?;

    /* for method in class.1.methods.public.iter() {
        if method.is_virtual {
            let mut out = String::new();
            method.to_cpp(&mut out)?;
            println!("{out}");
        }
    }*/

    /* let rust_modules: Vec<_> = modules
    .into_iter()
    .map(|module| RustModule::new(&class_hierarchy, module))
    .collect::<Result<Vec<_>>>()?;*/

    Ok(())
}
