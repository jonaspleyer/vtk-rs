use crate::Result;

type Path = Vec<String>;

#[derive(Debug, PartialEq)]
pub enum CppRawType {
    Void,
    NullPointer,
    SignedChar,
    UnsignedChar,
    ShortInt,
    UnsignedShortInt,
    Int,
    UnsignedInt,
    LongInt,
    UnsignedLongInt,
    LongLongInt,
    UnsignedLongLongInt,
    LongDouble,
    Double,
    Float,
    Array(Box<CppRawType>, usize),
    Vec(Box<CppRawType>),
    Map(Box<CppRawType>, Box<CppRawType>),
    LinkedList(Box<CppRawType>),
    Generic { pre: Path, args: Vec<CppRawType> },
    Path(Path),
}

#[derive(Debug, PartialEq)]
struct CppType {
    modifiers: Vec<Modifier>,
    r#type: CppRawType,
}

fn generic_args_regex() -> regex::Regex {
    regex::Regex::new(r#"([a-zA-Z0-9:]*)<(.*)>"#).unwrap()
}

fn split_into_arguments(input: &str, split_token: char) -> Vec<String> {
    // Return nothing if input is empty
    if input.trim().is_empty() {
        return vec![];
    }

    // Simply split at commata if  no brackets are present
    if !input.contains(">") && !input.contains("<") {
        return input
            .split(&split_token.to_string())
            .map(String::from)
            .collect();
    }

    let mut level = 0;

    let mut args = vec!["".to_string()];
    for char in input.chars() {
        if char == '<' {
            level += 1;
        } else if char == '>' {
            level -= 1;
        }
        if char == split_token && level == 0 {
            args.push("".to_string());
        } else {
            // We know that this element must be present. Thus we can safely unwrap.
            args.last_mut().unwrap().push(char);
        }
    }

    args.into_iter().filter(|x| !x.is_empty()).collect()
}

pub trait Parse: Sized {
    fn parse(input: &str) -> Result<Self>;
}

impl Parse for CppRawType {
    fn parse(input: &str) -> Result<Self> {
        let input = input.trim();

        if input.contains("<") && input.contains(">") {
            // It must be a generic
            let re = generic_args_regex();
            let segments = &anyhow::Context::context(
                re.captures(input),
                "Cannot parse empty genric arguments",
            )?;
            let pre = &segments[1];

            let args: Vec<_> = split_into_arguments(&segments[2], ',');
            match pre.trim() {
                "std::vector" | "vector" => {
                    let ty = CppRawType::parse(args[0].trim())?;
                    Ok(CppRawType::Vec(Box::new(ty)))
                }
                "std::array" | "array" => {
                    let ty = CppRawType::parse(args[0].trim())?;
                    use std::str::FromStr;
                    let n = usize::from_str(args[1].trim())?;
                    Ok(CppRawType::Array(Box::new(ty), n))
                }
                "std::map" | "map" => {
                    let key = CppRawType::parse(args[0].trim())?;
                    let value = CppRawType::parse(args[1].trim())?;
                    Ok(CppRawType::Map(Box::new(key), Box::new(value)))
                }
                "std::list" | "list" => {
                    let ty = CppRawType::parse(args[0].trim())?;
                    Ok(CppRawType::LinkedList(Box::new(ty)))
                }
                // Parse as some other not known generic
                _ => {
                    let pre = pre
                        .trim()
                        .split("::")
                        .filter(|x| !x.is_empty())
                        .map(String::from)
                        .collect();
                    let args = args
                        .into_iter()
                        .map(|x| CppRawType::parse(&x))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(CppRawType::Generic { pre, args })
                }
            }
        } else if input.contains("::") {
            // It must be some sort of path
            Ok(CppRawType::Path(
                input
                    .split("::")
                    .map(String::from)
                    .filter(|x| !x.is_empty())
                    .collect(),
            ))
        } else {
            use CppRawType::*;
            match input {
                "void" => Ok(Void),
                "signed char" => Ok(SignedChar),
                "short" | "short int" | "signed short" | "signed short int" => Ok(ShortInt),
                "int" | "signed" | "signed int" => Ok(Int),
                "long" | "long int" | "signed long" | "signed long int" => Ok(LongInt),
                "long long" | "long long int" | "signed long long" | "signed long long int" => {
                    Ok(LongLongInt)
                }
                "unsigned char" => Ok(UnsignedChar),
                "unsigned short" | "unsigned short int" => Ok(UnsignedShortInt),
                "unsigned" | "unsigned int" => Ok(UnsignedInt),
                "unsigned long" | "unsigned long int" => Ok(UnsignedLongInt),
                "unsigned long long" | "unsigned long long int" => Ok(UnsignedLongLongInt),
                "double" => Ok(Double),
                "long double" => Ok(LongDouble),
                "float" => Ok(Float),
                "char" => Ok(SignedChar),
                _ => anyhow::Context::context(None, format!("Cannot parse input: {}", input))?,
            }
        }
    }
}

impl Parse for CppType {
    fn parse(input: &str) -> Result<Self> {
        use anyhow::Context;

        let arg = input.replace("&", " & ");
        let arg = arg.replace("*", " * ");
        let arg = arg.trim();

        let mut modifiers = vec![];
        let mut cpp_type = None;
        let args = split_into_arguments(arg, ' ');
        for n in 0..args.len() {
            let current_segm = &args[n];

            if let Ok(modifier) = Modifier::parse(current_segm) {
                modifiers.push(modifier);
            } else {
                let mut final_args = vec![current_segm.clone()];
                for arg in args.iter().skip(n + 1) {
                    final_args.push(arg.clone());
                }
                cpp_type = Some(CppRawType::parse(&final_args.join(" "))?);
                break;
            }
        }

        Ok(CppType {
            modifiers,
            r#type: cpp_type.context("Could not find suitable type to parse")?,
        })
    }
}

impl CppType {
    fn parse_with_name(input: &str) -> Result<(String, Self)> {
        let arg = input.replace("&", " & ");
        let arg = arg.replace("*", " * ");

        todo!()

        // Ok((
        //     modifiers,
        //     cpp_type.context("Could not find function argument type")?,
        // ))
    }
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Ref,
    Pointer,
    Const,
    Volatile,
}

pub struct FunctionSignature {
    return_type: CppType,
    name: String,
    args: Vec<(String, CppType)>,
}

impl Parse for Modifier {
    fn parse(input: &str) -> Result<Self> {
        use Modifier::*;
        match input {
            "&" => Ok(Ref),
            "*" => Ok(Pointer),
            "const" => Ok(Const),
            "volatile" => Ok(Volatile),
            _ => anyhow::Context::context(None, "")?,
        }
    }
}

impl Parse for FunctionSignature {
    fn parse(input: &str) -> Result<Self> {
        let re = regex::Regex::new(r#"([a-zA-Z0-9_: ]*)\((.*)\)"#).unwrap();

        let segments = &anyhow::Context::context(
            re.captures(input),
            format!("Cannot parse function signature: {}", input),
        )?;
        let outer = segments[1].trim();
        let inner = segments[2].trim();
        let args = split_into_arguments(inner, ',')
            .into_iter()
            .map(|arg| CppType::parse_with_name(&arg))
            .collect::<Result<Vec<_>>>()?;

        let segments = outer.split(" ").collect::<Vec<_>>();
        let return_type = CppType::parse(segments[0])?;
        let name = segments[1].to_string();

        Ok(FunctionSignature {
            return_type,
            name,
            args,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_types() -> Result<()> {
        let t0 = "char";
        let cpp_type = CppRawType::parse(t0)?;
        assert_eq!(cpp_type, CppRawType::SignedChar);

        let t1 = "unsigned char";
        let cpp_type = CppRawType::parse(t1)?;
        assert_eq!(cpp_type, CppRawType::UnsignedChar);

        Ok(())
    }

    #[test]
    fn parse_array() -> Result<()> {
        macro_rules! parse_array(
            ($arr:ident, $cppty:path, $n:literal) => {
                let parsed = CppType::parse($arr)?;
                if let CppRawType::Array(ty, n) = parsed.r#type {
                    assert_eq!(n, $n);
                    match ty.as_ref() {
                        $cppty => (),
                        _ => panic!(),
                    }
                } else {
                    panic!();
                }
            }
        );

        let array1 = "std::array<float, 3>";
        parse_array!(array1, CppRawType::Float, 3);
        let array2 = "std::array<double, 12>";
        parse_array!(array2, CppRawType::Double, 12);
        let array3 = "array<int, 4>";
        parse_array!(array3, CppRawType::Int, 4);

        Ok(())
    }

    #[test]
    fn parse_map() -> Result<()> {
        macro_rules! parse_map(
            ($map:ident, $cppkey:path, $cppvalue:path) => {
                let parsed = CppType::parse($map)?;
                if let CppRawType::Map(key, value) = parsed.r#type {
                    match key.as_ref() {
                        $cppkey => (),
                        _ => panic!(),
                    }
                    match value.as_ref() {
                        $cppvalue => (),
                        _ => panic!(),
                    }
                } else {
                    panic!();
                }
            }
        );

        let map1 = "std::map<int, float>";
        parse_map!(map1, CppRawType::Int, CppRawType::Float);
        let map2 = "std::map<long, char>";
        parse_map!(map2, CppRawType::LongInt, CppRawType::SignedChar);
        let map3 = "map<unsigned char, double>";
        parse_map!(map3, CppRawType::UnsignedChar, CppRawType::Double);

        Ok(())
    }

    #[test]
    fn parse_list() -> Result<()> {
        macro_rules! parse_list(
            ($list:ident, $($cppty:tt)*) => {
                let parsed = CppType::parse($list)?;
                if let CppRawType::LinkedList(ty) = parsed.r#type {
                    match ty.as_ref() {
                        $($cppty)* => (),
                        _ => panic!(),
                    }
                } else {
                    panic!();
                }
            }
        );

        let list1 = "std::list<float>";
        parse_list!(list1, CppRawType::Float);
        let list2 = "std::list<char>";
        parse_list!(list2, CppRawType::SignedChar);
        let list3 = "std::list<unsigned char>";
        parse_list!(list3, CppRawType::UnsignedChar);
        let list4 = "std::list<map<int, char>>";
        parse_list!(list4, CppRawType::Map(_, _));

        Ok(())
    }

    #[test]
    fn parse_vec() -> Result<()> {
        macro_rules! parse_vec(
            ($vec:ident, $($cppty:tt)*) => {
                let parsed = CppType::parse($vec)?;
                if let CppRawType::Vec(ty) = parsed.r#type {
                    match ty.as_ref() {
                        $($cppty)* => (),
                        _ => panic!(),
                    }
                } else {
                    panic!();
                }
            }
        );

        let vec1 = "std::vector<long>";
        parse_vec!(vec1, CppRawType::LongInt);
        let vec2 = "std::vector<std::vector<int>>";
        parse_vec!(vec2, CppRawType::Vec(_));
        let vec3 = "vector<char>";
        parse_vec!(vec3, CppRawType::SignedChar);

        Ok(())
    }

    #[test]
    fn parse_path() -> Result<()> {
        macro_rules! parse_path(
            ($path:ident, $($segments:tt)*) => {
                let parsed = CppType::parse($path)?;
                if let CppRawType::Path(p) = parsed.r#type {
                    for (p1, p2) in p.into_iter().zip($($segments)*.into_iter()) {
                        assert!(p1 == p2);
                    }
                } else {
                    panic!();
                }
            }
        );

        let path1 = "namespace::function";
        parse_path!(path1, ["namespace", "function"]);
        let path2 = "std::vector";
        parse_path!(path2, ["std", "vector"]);
        let path3 = "::std::vector";
        parse_path!(path3, ["std", "vector"]);

        Ok(())
    }

    #[test]
    fn parse_generic() -> Result<()> {
        macro_rules! parse_generic(
            ($generic:ident, $pre:literal, [$($args:path),*]) => {
                let parsed = CppType::parse($generic)?;
                if let CppRawType::Generic {
                    pre,
                    args,
                } = parsed.r#type {
                    assert!(pre.join("::") == $pre);
                    let mut args = args.into_iter();
                    $(
                        match args.next().unwrap() {
                            $args => (),
                            _ => panic!(),
                        }
                    )*
                } else {
                    panic!();
                }
            }
        );

        let generic1 = "json<int, char>";
        parse_generic!(generic1, "json", [CppRawType::Int, CppRawType::SignedChar]);
        let generic2 = "what::the<unsigned char, double, int>";
        parse_generic!(
            generic2,
            "what::the",
            [
                CppRawType::UnsignedChar,
                CppRawType::Double,
                CppRawType::Int
            ]
        );

        Ok(())
    }
}
