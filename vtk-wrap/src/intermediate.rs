use std::collections::HashMap;

use crate::Result;
use crate::inheritance_hierarchy::{ClassHierarchy, ClassName};
use crate::parse_cpp::CppType;
use crate::parse_wrap_vtk_xml::Module;

pub struct RustMethod {
    pub name: String,
    pub return_type: CppType,
    pub args: Vec<(crate::parse_cpp::Ident, CppType)>,
}

pub struct RustStruct {
    pub name: String,
    pub parents: Vec<ClassName>,
    pub public_methods: Vec<RustMethod>,
}

pub struct RustModule {
    pub name: String,
    pub classes: HashMap<ClassName, RustStruct>,
}

impl RustModule {
    pub fn new(class_hierarchy: &ClassHierarchy, module: Module) -> Result<Self> {
        let name = module.name;
        let internal_objects = module
            .files
            .into_iter()
            .flat_map(|(_, file)| file.classes)
            .map(|class| {
                (
                    class.name.clone(),
                    RustStruct {
                        name: class.name.clone(),
                        parents: class_hierarchy
                            .get_parent_names(&class.name)
                            .into_iter()
                            .cloned()
                            .collect(),
                        public_methods: vec![],
                    },
                )
            })
            .collect();

        Ok(RustModule {
            name,
            classes: internal_objects,
        })
    }
}
