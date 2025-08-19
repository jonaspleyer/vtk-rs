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

impl TryFrom<crate::Method> for RustMethod {
    type Error = anyhow::Error;

    fn try_from(value: crate::Method) -> Result<Self> {
        use crate::parse_cpp::Parse;
        let return_type = if let Some(crate::ReturnType { ret_type, pointer }) = value.return_type {
            let inner_ty = CppType::parse(&ret_type)?;
            match pointer {
                Some(crate::Pointer::Ref) => CppType::Ref(Box::new(inner_ty)),
                Some(crate::Pointer::Star) => CppType::Pointer(Box::new(inner_ty)),
                Some(crate::Pointer::StarStar) => {
                    CppType::Pointer(Box::new(CppType::Ref(Box::new(inner_ty))))
                }
                None => inner_ty,
            }
        } else {
            CppType::Void
        };

        let args = value
            .parameters
            .into_iter()
            .enumerate()
            .map(|(n, param)| {
                let name = match param.name {
                    Some(name) => name,
                    // TODO make this better
                    None => format!("p{n}"),
                };
                let name = crate::parse_cpp::Ident::parse(&name)?;
                CppType::parse(&param.r#type).map(|x| (name, x))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(RustMethod {
            name: value.name,
            return_type,
            args,
        })
    }
}

pub struct RustStruct {
    pub name: String,
    pub parents: Vec<ClassName>,
    pub exposable_methods: Vec<RustMethod>,
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
                Ok((
                    class.name.clone(),
                    RustStruct {
                        name: class.name.clone(),
                        parents: class_hierarchy
                            .get_parent_names(&class.name)
                            .into_iter()
                            .cloned()
                            .collect(),
                        exposable_methods: class_hierarchy
                            .get_exposable_methods(&class.name)?
                            .into_iter()
                            .map(RustMethod::try_from)
                            .collect::<Result<Vec<_>>>()?,
                    },
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(RustModule {
            name,
            classes: internal_objects,
        })
    }
}
