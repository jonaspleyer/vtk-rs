use anyhow::{Context, Result};
use serde::Deserialize;

pub struct Module {
    pub name: String,
    pub path: std::path::PathBuf,
    pub files: Vec<(std::path::PathBuf, File)>,
}

unsafe impl Send for Module {}
unsafe impl Sync for Module {}

pub fn get_modules(path: impl Into<std::path::PathBuf>) -> Result<Vec<Module>> {
    use rayon::prelude::*;
    let path: std::path::PathBuf = path.into();
    let modules = glob::glob(&path.display().to_string())?.collect::<Result<Vec<_>, _>>()?;

    // Deserialize all modules
    let n_mods = modules.len();
    let n = std::sync::atomic::AtomicUsize::new(0);
    modules
        .into_par_iter()
        .map(|path| {
            let name = path
                .components()
                .next_back()
                .and_then(|x| x.as_os_str().to_str())
                .map(|x| x.to_string())
                .context("Could not determine last part of path")?;
            let files = glob::glob(
                path.join("*")
                    .to_str()
                    .context("could not convert path tot string")?,
            )?
            .map(|x| {
                let f = x?.to_path_buf();
                let mut file = std::fs::File::open(&f)?;
                let mut contents = String::new();
                use std::io::prelude::*;
                file.read_to_string(&mut contents)?;
                let x: File = serde_xml_rs::SerdeXml::new()
                    .overlapping_sequences(true)
                    .from_str(&contents)?;

                Ok((f, x))
            })
            .collect::<Result<_>>()?;

            n.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            println!(
                "[{:3.0}%] {}",
                n.load(std::sync::atomic::Ordering::Relaxed) as f32 / n_mods as f32 * 100.,
                name
            );

            Ok(Module { name, path, files })
        })
        .collect::<Result<_>>()
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum Access {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "protected")]
    Protected,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ReturnType {
    #[serde(rename = "@type")]
    pub ret_type: String,
    #[serde(rename = "@pointer")]
    pub pointer: Option<Pointer>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename = "base")]
pub struct Base {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@access")]
    pub access: Access,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename = "method")]
pub struct Method {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@property")]
    pub property: Option<String>,
    #[serde(rename = "@access")]
    pub access: Option<Access>,
    #[serde(rename = "@const")]
    pub is_const: Option<u8>,
    #[serde(rename = "@virtual")]
    pub is_virtual: Option<u8>,
    pub signature: String,
    pub comment: Option<String>,
    #[serde(rename = "return")]
    pub return_type: Option<ReturnType>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct TypeDef {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@access")]
    access: Access,
    #[serde(rename = "@type")]
    r#type: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Property {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@access")]
    pub access: Access,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@pointer")]
    pub pointer: Option<Pointer>,
    pub comment: Option<String>,
    #[serde(default = "Vec::new")]
    pub methods: Vec<PropertyMethods>,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, PartialEq, Debug)]
pub enum Bitfield {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "SET")]
    Set,
    #[serde(rename = "GET|SET")]
    Get_Set,
    #[serde(rename = "GET|SET|SET_BOOL")]
    Get_Set_SetBool,
    #[serde(rename = "SET_IDX")]
    SetIdx,
    #[serde(rename = "GET_IDX")]
    GetIdx,
    #[serde(rename = "GET|GET_IDX")]
    Get_GetIdx,
    #[serde(rename = "GET_RHS")]
    GetRhs,
    #[serde(rename = "GET|GET_RHS")]
    Get_GetRhs,
    #[serde(rename = "GET_IDX_RHS")]
    GetIdxRhs,
    #[serde(rename = "GET_IDX|GET_IDX_RHS")]
    GetIdx_GetIdxRhs,
    // ...
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct PropertyMethods {
    #[serde(rename = "@bitfield")]
    pub bitfield: String,
    #[serde(rename = "@access")]
    pub access: Access,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct CContext {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@access")]
    pub access: Access,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Inheritance {
    pub context: Vec<CContext>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum Pointer {
    #[serde(rename = "&")]
    Ref,
    #[serde(rename = "*")]
    Star,
    #[serde(rename = "**")]
    StarStar,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Member {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@access")]
    pub access: Access,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@pointer")]
    pub pointer: Option<Pointer>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename = "class")]
pub struct Class {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@access")]
    pub access: Option<Access>,
    #[serde(rename = "@abstract")]
    pub is_abstract: Option<u8>,
    pub comment: Option<String>,
    #[serde(default = "Vec::new")]
    pub base: Vec<Base>,
    pub inheritance: Option<Inheritance>,
    #[serde(rename = "method")]
    #[serde(default = "Vec::new")]
    pub methods: Vec<Method>,
    #[serde(rename = "typedef")]
    #[serde(default = "Vec::new")]
    pub typedefs: Vec<TypeDef>,
    #[serde(rename = "property")]
    #[serde(default = "Vec::new")]
    pub properties: Vec<Property>,
    #[serde(default = "Vec::new")]
    pub members: Vec<Member>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct File {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "class")]
    #[serde(default = "Vec::new")]
    pub classes: Vec<Class>,
}

#[cfg(test)]
mod test_parsing {
    use super::*;

    const METHOD: &str = r#"
<method
    name="GetClassNameInternal"
    property="ClassNameInternal"
    access="protected"
    const="1"
    virtual="1"
>
    <signature>
            virtual const char *GetClassNameInternal() const
    </signature>
    <comment>
            Return the class name as a string. This method is overridden
            in all subclasses of vtkObjectBase with the vtkTypeMacro found
            in vtkSetGet.h.
    </comment>
    <return type="const char" pointer="*" />
</method>"#;

    #[test]
    fn parse_method() -> Result<()> {
        let Method {
            name,
            property,
            access,
            is_const,
            is_virtual,
            signature,
            comment,
            return_type,
        } = serde_xml_rs::SerdeXml::new()
            .overlapping_sequences(true)
            .from_str(METHOD)?;
        assert_eq!(name, "GetClassNameInternal");
        assert_eq!(property.unwrap(), "ClassNameInternal");
        assert_eq!(access, Some(Access::Protected));
        assert_eq!(is_const, Some(1));
        assert_eq!(is_virtual, Some(1));
        assert_eq!(
            signature,
            "virtual const char *GetClassNameInternal() const"
        );
        assert_eq!(
            comment.unwrap(),
            "\
            Return the class name as a string. This method is overridden
            in all subclasses of vtkObjectBase with the vtkTypeMacro found
            in vtkSetGet.h."
        );
        assert_eq!(
            return_type,
            Some(ReturnType {
                ret_type: "const char".into(),
                pointer: Some(Pointer::Star)
            })
        );

        Ok(())
    }

    const PROPERTY: &str = r#"
<property name="ClassNameInternal" access="protected" type="char" pointer="*">
    <comment>
            Return the class name as a string. This method is overridden
            in all subclasses of vtkObjectBase with the vtkTypeMacro found
            in vtkSetGet.h.
    </comment>
    <methods bitfield="GET" access="protected" />
</property>"#;

    #[test]
    fn parse_property() -> Result<()> {
        let Property {
            name,
            access,
            r#type,
            pointer,
            comment,
            methods,
        } = serde_xml_rs::SerdeXml::new()
            .overlapping_sequences(true)
            .from_str(PROPERTY)?;

        assert_eq!(name, "ClassNameInternal");
        assert_eq!(access, Access::Protected);
        assert_eq!(r#type, "char");
        assert_eq!(pointer, Some(Pointer::Star));
        assert_eq!(
            comment.unwrap(),
            "\
            Return the class name as a string. This method is overridden
            in all subclasses of vtkObjectBase with the vtkTypeMacro found
            in vtkSetGet.h."
        );
        assert_eq!(
            methods,
            vec![PropertyMethods {
                bitfield: "GET".into(),
                access: Access::Protected
            }]
        );

        Ok(())
    }

    const CLASS: &str = r#"
<class name="vtkChart" abstract="1">
  <comment>
     @brief   Factory class for drawing 2D charts


     This defines the interface for a chart.
  </comment>
  <base name="vtkContextItem" access="public" />

  <inheritance>
    <context name="vtkContextItem" access="public" />
    <context name="vtkAbstractContextItem" access="public" />
    <context name="vtkObject" access="public" />
    <context name="vtkObjectBase" access="public" />
  </inheritance>

  <property name="ClassNameInternal" access="protected" type="char" pointer="*">
    <comment>
       Return the class name as a string. This method is overridden
       in all subclasses of vtkObjectBase with the vtkTypeMacro found
       in vtkSetGet.h.
    </comment>
    <methods bitfield="GET" access="protected" />
  </property>

  <method name="GetClassNameInternal" property="ClassNameInternal" access="protected" const="1" virtual="1">
    <signature>
       virtual const char *GetClassNameInternal() const
    </signature>
    <comment>
       Return the class name as a string. This method is overridden
       in all subclasses of vtkObjectBase with the vtkTypeMacro found
       in vtkSetGet.h.
    </comment>
    <return type="const char" pointer="*" />
  </method>

  <typedef name="Superclass" access="public" type="vtkContextItem" />
</class>"#;

    #[test]
    fn parse_class() -> Result<()> {
        let Class {
            name,
            access,
            is_abstract,
            comment,
            base,
            inheritance,
            methods,
            typedefs,
            properties,
            members,
        } = serde_xml_rs::SerdeXml::new()
            .overlapping_sequences(true)
            .from_str(CLASS)?;

        assert_eq!(name, "vtkChart");
        assert_eq!(access, None);
        assert_eq!(is_abstract, Some(1));
        assert_eq!(
            comment.unwrap(),
            "\
     @brief   Factory class for drawing 2D charts


     This defines the interface for a chart."
        );
        assert_eq!(
            base,
            vec![Base {
                name: "vtkContextItem".into(),
                access: Access::Public
            }]
        );
        assert_eq!(
            inheritance.unwrap(),
            Inheritance {
                context: vec![
                    CContext {
                        name: "vtkContextItem".into(),
                        access: Access::Public
                    },
                    CContext {
                        name: "vtkAbstractContextItem".into(),
                        access: Access::Public
                    },
                    CContext {
                        name: "vtkObject".into(),
                        access: Access::Public
                    },
                    CContext {
                        name: "vtkObjectBase".into(),
                        access: Access::Public
                    },
                ]
            }
        );
        assert_eq!(
            methods,
            vec![Method {
                name: "GetClassNameInternal".into(),
                property: Some("ClassNameInternal".into()),
                access: Some(Access::Protected),
                is_const: Some(1),
                signature: "virtual const char *GetClassNameInternal() const".into(),
                is_virtual: Some(1),
                comment: Some(
                    "\
       Return the class name as a string. This method is overridden
       in all subclasses of vtkObjectBase with the vtkTypeMacro found
       in vtkSetGet.h."
                        .into(),
                ),
                return_type: Some(ReturnType {
                    ret_type: "const char".into(),
                    pointer: Some(Pointer::Star),
                }),
            }]
        );
        assert_eq!(
            typedefs,
            vec![TypeDef {
                name: "Superclass".into(),
                access: Access::Public,
                r#type: "vtkContextItem".into()
            }]
        );
        assert_eq!(
            properties,
            vec![Property {
                name: "ClassNameInternal".into(),
                access: Access::Protected,
                r#type: "char".into(),
                pointer: Some(Pointer::Star),
                comment: Some(
                    "\
       Return the class name as a string. This method is overridden
       in all subclasses of vtkObjectBase with the vtkTypeMacro found
       in vtkSetGet.h."
                        .into()
                ),
                methods: vec![PropertyMethods {
                    bitfield: "GET".into(),
                    access: Access::Protected
                }]
            }]
        );
        assert_eq!(members, vec![]);

        Ok(())
    }
}
