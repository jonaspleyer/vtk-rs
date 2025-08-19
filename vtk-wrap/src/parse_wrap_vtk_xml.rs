use anyhow::{Context, Result};
use log::info;
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
    info!("Reading Modules");
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
            info!(
                "[{:3.0}%] {}",
                n.load(std::sync::atomic::Ordering::Relaxed) as f32 / n_mods as f32 * 100.,
                name
            );

            Ok(Module { name, path, files })
        })
        .collect::<Result<_>>()
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum Access {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "protected")]
    Protected,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ReturnType {
    #[serde(rename = "@type")]
    pub ret_type: String,
    #[serde(rename = "@pointer")]
    pub pointer: Option<Pointer>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "base")]
pub struct Base {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@access")]
    pub access: Access,
}

fn option_one_to_bool<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    struct Vis;
    impl<'de> serde::de::Visitor<'de> for Vis {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("expected Option<u8>")
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v == "1" { Ok(true) } else { Ok(false) }
        }
    }
    de.deserialize_string(Vis)
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "method")]
pub struct Method {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@property")]
    pub property: Option<String>,
    #[serde(rename = "@access")]
    pub access: Access,
    #[serde(rename = "@const")]
    #[serde(default = "Default::default")]
    #[serde(deserialize_with = "option_one_to_bool")]
    pub is_const: bool,
    #[serde(rename = "@static")]
    #[serde(default = "Default::default")]
    #[serde(deserialize_with = "option_one_to_bool")]
    pub is_static: bool,
    #[serde(rename = "@virtual")]
    #[serde(default = "Default::default")]
    #[serde(deserialize_with = "option_one_to_bool")]
    pub is_virtual: bool,
    // #[serde(deserialize_with = "deserialize_signature")]
    pub signature: String,
    #[serde(rename = "param")]
    #[serde(default = "Vec::new")]
    pub parameters: Vec<Parameter>,
    pub comment: Option<String>,
    #[serde(rename = "return")]
    pub return_type: Option<ReturnType>,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Methods {
    pub private: Vec<Method>,
    pub public: Vec<Method>,
    pub protected: Vec<Method>,
}

fn deserialize_methods<'de, D>(de: D) -> Result<Methods, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = Methods;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Exptected list of methods")
        }

        fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut private = vec![];
            let mut public = vec![];
            let mut protected = vec![];

            while let Some(method) = seq.next_element::<Method>()? {
                match method.access {
                    Access::Protected => protected.push(method),
                    Access::Public => public.push(method),
                    Access::Private => private.push(method),
                }
            }

            Ok(Methods {
                private,
                public,
                protected,
            })
        }
    }

    de.deserialize_seq(Visitor)
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "param")]
pub struct Parameter {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@reference")]
    #[serde(default = "Default::default")]
    #[serde(deserialize_with = "option_one_to_bool")]
    pub reference: bool,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct TypeDef {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@access")]
    access: Access,
    #[serde(rename = "@type")]
    r#type: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
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
#[derive(Deserialize, PartialEq, Debug, Clone)]
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

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct PropertyMethods {
    #[serde(rename = "@bitfield")]
    pub bitfield: String,
    #[serde(rename = "@access")]
    pub access: Access,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct CContext {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@access")]
    pub access: Access,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Inheritance {
    pub context: Vec<CContext>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum Pointer {
    #[serde(rename = "&")]
    Ref,
    #[serde(rename = "*")]
    Star,
    #[serde(rename = "**")]
    StarStar,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
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

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename = "class")]
pub struct Class {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@abstract")]
    #[serde(default = "Default::default")]
    #[serde(deserialize_with = "option_one_to_bool")]
    pub is_abstract: bool,
    pub comment: Option<String>,
    #[serde(default = "Vec::new")]
    pub base: Vec<Base>,
    #[serde(default = "Default::default")]
    pub inheritance: Option<Inheritance>,
    #[serde(rename = "method")]
    #[serde(deserialize_with = "deserialize_methods")]
    #[serde(default = "Default::default")]
    pub methods: Methods,
    #[serde(rename = "typedef")]
    #[serde(default = "Vec::new")]
    pub typedefs: Vec<TypeDef>,
    #[serde(rename = "property")]
    #[serde(default = "Vec::new")]
    pub properties: Vec<Property>,
    #[serde(default = "Vec::new")]
    pub members: Vec<Member>,
}

impl Class {
    pub fn combine(&mut self, other: &Self) -> Result<()> {
        let Class {
            name,
            is_abstract,
            comment,
            base,
            inheritance,
            methods,
            typedefs,
            properties,
            members,
        } = self;

        let cmp_inheritence = match (inheritance, &other.inheritance) {
            (Some(i1), Some(i2)) => i1.context.iter().any(|c| !i2.context.contains(c)),
            (None, Some(_)) | (Some(_), None) => false,
            (None, None) => true,
        };
        if *name != other.name
            || *is_abstract != other.is_abstract
            || base.iter().any(|b| !other.base.contains(b))
            || *comment != other.comment
            || cmp_inheritence
        {
            return Err(anyhow::anyhow!(
                "Properties of Classes which should be combined are not matching"
            ));
        }

        macro_rules! add_new(
            ($($to:tt)*) => {
                let new_methods = other
                    .$($to)*
                    .iter()
                    .filter(|method| !$($to)* .contains(method))
                    .cloned()
                    .collect::<Vec<_>>();

                $($to)* .extend(new_methods);
            }
        );

        add_new!(methods.public);
        add_new!(methods.private);
        add_new!(methods.protected);
        add_new!(typedefs);
        add_new!(properties);
        add_new!(members);

        Ok(())
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
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

    const METHOD2: &str = r#"
<method name="PrintSelf" access="public" virtual="1">
    <signature>
            virtual void PrintSelf(ostream &amp;os, vtkIndent indent)
    </signature>
    <comment>
            Methods invoked by print to print information about the object
            including superclasses. Typically not called by the user (use
            Print() instead) but used in the hierarchical print process to
            combine the output of several classes.
    </comment>
    <param name="os" type="ostream" reference="1" />
    <param name="indent" type="vtkIndent" />
    <return type="void" />
</method>"#;

    #[test]
    fn parse_method() -> Result<()> {
        let Method {
            name,
            property,
            access,
            is_const,
            is_static,
            is_virtual,
            signature,
            parameters,
            comment,
            return_type,
        } = serde_xml_rs::SerdeXml::new()
            .overlapping_sequences(true)
            .from_str(METHOD)?;
        assert_eq!(name, "GetClassNameInternal");
        assert_eq!(property.unwrap(), "ClassNameInternal");
        assert_eq!(access, Access::Protected);
        assert!(is_const);
        assert!(!is_static);
        assert!(is_virtual);
        assert_eq!(
            signature,
            "virtual const char *GetClassNameInternal() const"
        );
        assert_eq!(parameters, vec![]);
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

    #[test]
    fn parse_method_2() -> Result<()> {
        let Method {
            name,
            property,
            access,
            is_const,
            is_static,
            is_virtual,
            signature,
            parameters,
            comment,
            return_type,
        } = serde_xml_rs::SerdeXml::new()
            .overlapping_sequences(true)
            .from_str(METHOD2)?;
        assert_eq!(name, "PrintSelf");
        assert_eq!(property, None);
        assert_eq!(access, Access::Public);
        assert!(!is_static);
        assert!(!is_const);
        assert!(is_virtual);
        assert!(is_virtual);
        assert_eq!(
            signature,
            "virtual void PrintSelf(ostream &os, vtkIndent indent)"
        );
        assert_eq!(
            comment.unwrap(),
            "\
            Methods invoked by print to print information about the object
            including superclasses. Typically not called by the user (use
            Print() instead) but used in the hierarchical print process to
            combine the output of several classes."
        );
        assert_eq!(
            parameters,
            vec![
                Parameter {
                    name: Some("os".into()),
                    r#type: "ostream".into(),
                    reference: true,
                },
                Parameter {
                    name: Some("indent".into()),
                    r#type: "vtkIndent".into(),
                    reference: false,
                }
            ]
        );
        assert_eq!(
            return_type,
            Some(ReturnType {
                ret_type: "void".into(),
                pointer: None
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
        assert!(is_abstract);
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
            methods.protected,
            vec![Method {
                name: "GetClassNameInternal".into(),
                property: Some("ClassNameInternal".into()),
                access: Access::Protected,
                is_const: true,
                is_static: false,
                signature: "virtual const char *GetClassNameInternal() const".into(),
                parameters: vec![],
                is_virtual: true,
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
        assert_eq!(methods.public, vec![]);
        assert_eq!(methods.private, vec![]);
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
