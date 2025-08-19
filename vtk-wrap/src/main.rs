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

    // let generator = Generator::new(&modules)?;

    /* for module in modules.iter() {
        for (_, file) in module.files.iter() {
            for class in file.classes.iter() {
                for method in class
                    .methods
                    .public
                    .iter()
                    .filter(|m| m.access == Access::Public && !m.signature.contains("template"))
                {
                    if let Some(ReturnType { ret_type, pointer }) = &method.return_type {
                        let ty = CppType::parse(ret_type)?;
                        match pointer {
                            Some(Pointer::Ref) => println!("&{:?}", CppType::Ref(Box::new(ty))),
                            Some(Pointer::Star) => println!("{:?}", CppType::Pointer(Box::new(ty))),
                            Some(Pointer::StarStar) => println!(
                                "{:?}",
                                CppType::Pointer(Box::new(CppType::Pointer(Box::new(ty))))
                            ),
                            None => println!("{ty:?}"),
                        }
                    }
                    for param in &method.parameters {
                        let Parameter {
                            name,
                            r#type,
                            reference,
                        } = param;
                        let ty = CppType::parse(r#type)?;
                    }
                }
            }
        }
    }*/

    Ok(())
}
