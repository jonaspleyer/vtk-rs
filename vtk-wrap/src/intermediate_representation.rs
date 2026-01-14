use std::collections::HashMap;

use crate::Result;
use crate::inheritance_hierarchy::{ClassHierarchy, ClassName};
use crate::parse_cpp::CppType;
use crate::parse_wrap_vtk_xml::{Access, Module};

#[allow(non_camel_case_types)]
pub enum IRType {
    /// Type `()` or `void` in C++
    Unit,
    c_char,
    c_short,
    c_int,
    c_long,
    c_longlong,
    c_uchar,
    c_ushort,
    c_uint,
    c_ulong,
    c_ulonglong,
    bool,
    float,
    double,
    usize,
    File,
    FileMode,
    Ref(Box<IRType>),
    Pointer(Box<IRType>),
    Const(Box<IRType>),
    String,
    Array(Box<IRType>, usize),
    Vec(Box<IRType>),
    Map(Box<IRType>, Box<IRType>),
    LinkedList(Box<IRType>),
    Path(crate::parse_cpp::Path),
}

impl TryFrom<&CppType> for IRType {
    type Error = anyhow::Error;

    fn try_from(value: &CppType) -> std::result::Result<Self, Self::Error> {
        use CppType::*;
        use IRType::*;
        let res = match value {
            Void => Unit,
            SignedChar => c_char,
            UnsignedChar => c_uchar,
            ShortInt => c_short,
            UnsignedShortInt => c_ushort,
            Int => c_int,
            UnsignedInt => c_uint,
            LongInt => c_long,
            UnsignedLongInt => c_ulong,
            LongLongInt => c_longlong,
            UnsignedLongLongInt => c_ulonglong,
            LongDouble => anyhow::bail!("Long Double not supported yet"),
            Double => double,
            Float => float,
            Bool => bool,
            Ostream => anyhow::bail!("Ostream not supported yet"),
            SizeT => usize,
            CppType::File => IRType::File,
            CppType::FileMode => IRType::FileMode, // anyhow::bail!("Filemode not supported yet"),
            Function(_) => anyhow::bail!("Function not supported yet"),
            CppType::Ref(cpp_box) => IRType::Ref(Box::new(IRType::try_from(cpp_box.as_ref())?)),
            CppType::Pointer(cpp_box) => {
                IRType::Pointer(Box::new(IRType::try_from(cpp_box.as_ref())?))
            }
            CppType::Const(cpp_box) => IRType::Const(Box::new(IRType::try_from(cpp_box.as_ref())?)),
            CppType::String => IRType::String,
            CppType::Array(cpp_box, size) => {
                IRType::Array(Box::new(IRType::try_from(cpp_box.as_ref())?), *size)
            }
            CppType::Vec(cpp_box) => IRType::Vec(Box::new(IRType::try_from(cpp_box.as_ref())?)),
            CppType::Map(cpp_box1, cpp_box2) => IRType::Map(
                Box::new(IRType::try_from(cpp_box1.as_ref())?),
                Box::new(IRType::try_from(cpp_box2.as_ref())?),
            ),
            CppType::Path(p) => IRType::Path(crate::parse_cpp::Path(p.0.clone())),
            CppType::LinkedList(cpp_box) => {
                IRType::LinkedList(Box::new(IRType::try_from(cpp_box.as_ref())?))
            }
            CppType::Generic { pre: _, args: _ } => anyhow::bail!("Generics not supported yet"),
            CppType::TypeInfo => anyhow::bail!("TypeInfo not supported yet"),
        };
        Ok(res)
    }
}

pub struct IRIdent(pub String);

impl From<crate::parse_cpp::Ident> for IRIdent {
    fn from(value: crate::parse_cpp::Ident) -> Self {
        Self(value.0)
    }
}

pub struct IRMethod {
    pub name: String,
    pub return_type: IRType,
    pub args: Vec<(IRIdent, IRType)>,
}

impl IRMethod {
    fn convert_from_class(class: &crate::Class, value: &crate::Method) -> Result<Self> {
        use crate::parse_cpp::Parse;
        let return_type = if let Some(crate::ReturnType { ret_type, pointer }) = &value.return_type
        {
            let inner_ty = CppType::parse(ret_type)?;
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
        let return_type = IRType::try_from(&return_type)?;

        let args = value
            .parameters
            .iter()
            .enumerate()
            .map(|(n, param)| {
                let name = match &param.name {
                    Some(name) => name.clone(),
                    // TODO make this better
                    None => format!("p{n}"),
                };
                let name = crate::parse_cpp::Ident::parse(&name)?;
                let name = IRIdent::from(name);
                let cpp_ty = CppType::parse(&param.r#type)?;
                Ok((name, IRType::try_from(&cpp_ty)?))
            })
            .collect::<Result<Vec<_>>>()?;

        use convert_case::*;
        Ok(IRMethod {
            name: format!("{}_{}", class.name, value.name).to_case(Case::Snake),
            return_type,
            args,
        })
    }
}

pub struct IRStruct {
    pub name: String,
    pub description: Vec<String>,
    pub parents: Vec<ClassName>,
    pub exposable_methods: Vec<IRMethod>,
    pub is_abstract: bool,
    pub is_template: bool,
    pub filename: String,
    constructors: Vec<crate::parse_wrap_vtk_xml::Constructor>,
    destructors: Vec<crate::parse_wrap_vtk_xml::Destructor>,
}

impl IRStruct {
    pub fn constructor_binding_name(&self) -> String {
        format!("{}_new", self.name)
    }

    pub fn destructor_binding_name(&self) -> String {
        format!("{}_destructor", self.name)
    }

    pub(crate) fn is_constructable(&self) -> bool {
        self.constructors
            .iter()
            .any(|c| c.access != Access::Private)
            && self.destructors.iter().any(|d| d.access != Access::Private)
            && !self.is_abstract
            && !self.is_template
            && !self.exposable_methods.is_empty()
            // This could be lifted in the future when considering objects which can be constructed
            // not via vtkNew<..>()
            && self.parents.iter().any(|c| c == "vtkObjectBase")
    }
}

pub struct IRModule {
    pub name: String,
    pub classes: HashMap<ClassName, IRStruct>,
}

impl IRModule {
    pub(crate) fn contains_exposable_content(&self) -> bool {
        self.classes.iter().any(|x| x.1.is_constructable())
    }

    pub fn new(module: Module, class_hierarchy: &ClassHierarchy) -> Result<Self> {
        let name = module.name;
        let internal_objects = module
            .files
            .into_iter()
            .flat_map(|(_, file)| file.classes.into_iter().map(move |x| (file.name.clone(), x)))
            .map(|(filename, class)| {
                Ok((
                    class.name.clone(),
                    IRStruct {
                        name: class.name.clone(),
                        description: class.comment.as_ref()
                            .map(|x| x
                                .lines()
                                .map(|x| x
                                    .trim()
                                    .to_string()
                                ).collect::<Vec<String>>()
                            )
                            .unwrap_or(vec![]),
                        parents: class_hierarchy
                            .get_parent_names(&class.name)
                            .into_iter()
                            .cloned()
                            .collect(),
                        exposable_methods: class_hierarchy
                            .get_exposable_methods(&class.name)?
                            .into_iter()
                            .filter_map(|method| {
                                match IRMethod::convert_from_class(&class, &method) {
                                    Ok(x) => Some(x),
                                    Err(e) => {
                                        log::warn!(
                                            "[IR] Skipping method \"{}\" with signature \"{}\" of class \"{}\" due to error: \"{e}\"",
                                            method.name,
                                            method.signature,
                                            class.name,
                                        );
                                        None
                                    }
                                }
                            })
                            .collect::<Vec<_>>(),
                        is_abstract: class.is_abstract,
                        is_template: class.is_template,
                        filename: filename.clone(),
                        constructors: class.constructors,
                        destructors: class.destructors,
                    },
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(IRModule {
            name,
            classes: internal_objects,
        })
    }
}
