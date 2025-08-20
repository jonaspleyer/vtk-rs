use anyhow::Result;

mod code_gen;
mod inheritance_hierarchy;
mod intermediate;
mod parse_cpp;
mod parse_wrap_vtk_xml;

use inheritance_hierarchy::*;
use intermediate::*;
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
