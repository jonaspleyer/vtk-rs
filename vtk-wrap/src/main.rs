use anyhow::Result;

mod code_gen;
mod inheritance_hierarchy;
mod parse_cpp;
mod parse_wrap_vtk_xml;

use code_gen::*;
use parse_cpp::*;
use parse_wrap_vtk_xml::*;

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Obtain all modules
    let modules = get_modules("WrapVTK/build/xml/*CommonCore")?;

    let generator = Generator::new(&modules)?;

    for module in modules.iter() {
        for (_, file) in module.files.iter() {
            for class in file.classes.iter() {
                if let Some(trait_code) = generator.generate_trait(class)? {
                    // TODO
                }
            }
        }
    }
    Ok(())
}
