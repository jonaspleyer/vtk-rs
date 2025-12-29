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

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Obtain all modules
    let modules = get_modules("WrapVTK/build/xml/*CommonCore")?;

    let class_hierarchy = ClassHierarchy::new(&modules)?;
    let rust_modules: Vec<_> = modules
        .into_iter()
        .map(|module| RustModule::new(&class_hierarchy, module))
        .collect::<Result<Vec<_>>>()?;

    for rm in rust_modules {
        println!("{}", quote::quote!(#rm));
    }

    Ok(())
}
