use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

struct Module {
    name: String,
    path: std::path::PathBuf,
    files: Vec<std::path::PathBuf>,
}

unsafe impl Send for Module {}
unsafe impl Sync for Module {}

fn get_modules(path: &std::path::Path) -> Result<Vec<Module>> {
    let modules = glob::glob(&format!("{}/*", path.display()))?;
    modules
        .into_iter()
        .map(|m| {
            let path = m?;
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
            .map(|x| Ok(x?.to_path_buf()))
            .collect::<Result<_>>()?;
            Ok(Module { name, path, files })
        })
        .collect::<Result<_>>()
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Access {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "protected")]
    Protected,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ReturnType {
    #[serde(rename = "@type")]
    ret_type: String,
    #[serde(rename = "@pointer")]
    pointer: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename = "base")]
struct Base {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@access")]
    access: Access,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename = "method")]
struct Method {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@property")]
    property: Option<String>,
    #[serde(rename = "@access")]
    access: Option<Access>,
    #[serde(rename = "@const")]
    is_const: Option<u8>,
    signature: String,
    comment: Option<String>,
    #[serde(rename = "return")]
    return_type: Option<ReturnType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TypeDef {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@access")]
    access: Access,
    #[serde(rename = "@type")]
    r#type: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Property {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@access")]
    access: Access,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@pointer")]
    pointer: Option<String>,
    comment: Option<String>,
    #[serde(default = "Vec::new")]
    methods: Vec<PropertyMethods>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Bitfield {
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct PropertyMethods {
    #[serde(rename = "@bitfield")]
    bitfield: String,
    #[serde(rename = "@access")]
    access: Access,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct CContext {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@access")]
    access: Access,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Inheritance {
    context: Vec<CContext>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Member {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@access")]
    access: Access,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@value")]
    value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename = "class")]
struct CClass {
    #[serde(default = "Vec::new")]
    base: Vec<Base>,
    #[serde(default = "Vec::new")]
    inheritance: Vec<Inheritance>,
    #[serde(rename = "method")]
    #[serde(default = "Vec::new")]
    methods: Vec<Method>,
    #[serde(rename = "typedef")]
    #[serde(default = "Vec::new")]
    typedefs: Vec<TypeDef>,
    #[serde(rename = "property")]
    #[serde(default = "Vec::new")]
    properties: Vec<Property>,
    #[serde(default = "Vec::new")]
    members: Vec<Member>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct File {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "class")]
    #[serde(default = "Vec::new")]
    classes: Vec<CClass>,
}

fn main() -> Result<()> {
    // Obtain all modules
    let modules = get_modules(&std::path::PathBuf::from_iter(["WrapVTK", "build", "xml"]))?;

    // Deserialize all modules
    let n_mods = modules.len();
    let n = std::sync::atomic::AtomicUsize::new(0);
    use rayon::prelude::*;
    let results = modules
        .into_par_iter()
        .map(|m| {
            let mut files = vec![];
            for f in m.files.iter() {
                let mut file = std::fs::File::open(f)?;
                let mut contents = String::new();
                use std::io::prelude::*;
                file.read_to_string(&mut contents)?;
                let x: File = serde_xml_rs::SerdeXml::new()
                    .overlapping_sequences(true)
                    .from_str(&contents)?;
                files.push(x);
            }
            n.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            println!(
                "[{:3.0}%] {}",
                (n.load(std::sync::atomic::Ordering::Relaxed) + 1) as f32 / n_mods as f32 * 100.,
                m.name
            );
            Ok(files)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(())
}
