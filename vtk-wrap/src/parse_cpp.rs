use crate::Result;

type Path = Vec<String>;

#[derive(Debug, PartialEq)]
pub enum CppType {
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
    Bool,
    Ostream,
    SizeT,
    File,
    FileMode,
    Function(Box<StdFunction>),
    // Containers
    Ref(Box<CppType>),
    Pointer(Box<CppType>),
    Const(Box<CppType>),
    // Standard Library
    String,
    TypeInfo,
    Array(Box<CppType>, usize),
    Vec(Box<CppType>),
    Map(Box<CppType>, Box<CppType>),
    LinkedList(Box<CppType>),
    // Others
    Generic { pre: Path, args: Vec<CppType> },
    Path(Path),
}

fn generic_args_regex() -> regex::Regex {
    regex::Regex::new(r#"([a-zA-Z0-9:]*)<(.*)>"#).unwrap()
}

fn split_into_arguments(input: &str, bra: char, ket: char, split_token: char) -> Vec<String> {
    // Return nothing if input is empty
    if input.trim().is_empty() {
        return vec![];
    }

    // Simply split at commata if  no brackets are present
    if !input.contains(bra) && !input.contains(ket) {
        return input
            .split(&split_token.to_string())
            .map(String::from)
            .collect();
    }

    let mut level = 0;

    let mut args = vec!["".to_string()];
    for char in input.chars() {
        if char == bra {
            level += 1;
        } else if char == ket {
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

impl Parse for CppType {
    fn parse(input: &str) -> Result<Self> {
        let input = input.trim();

        let first_char = input.chars().next();
        let last_char = input.chars().last();
        let remaining_last_chars = input.chars().skip(1).collect::<String>();
        let remaining_first_chars = {
            let n = input.len().saturating_sub(1);
            input.chars().take(n).collect::<String>()
        };
        let first_word = input.split(" ").next();
        let last_word = input.split(" ").last();
        let remaining_words = input.split(" ").skip(1).collect::<Vec<_>>().join(" ");
        let initial_words = {
            let i = input.split(" ").collect::<Vec<_>>();
            let n = i.len() - 1;
            i.into_iter().take(n).collect::<Vec<_>>().join(" ")
        };

        if let Some("const") = first_word {
            let inner = CppType::parse(&remaining_words)?;
            Ok(CppType::Const(Box::new(inner)))
        } else if let Some("const") = last_word {
            let inner = CppType::parse(&initial_words)?;
            Ok(CppType::Const(Box::new(inner)))
        } else if let Some('&') = first_char {
            let tail = CppType::parse(&remaining_last_chars)?;
            Ok(CppType::Ref(Box::new(tail)))
        } else if let Some('&') = last_char {
            let head = CppType::parse(&remaining_first_chars)?;
            Ok(CppType::Ref(Box::new(head)))
        } else if let Some('*') = last_char {
            let head = CppType::parse(&remaining_first_chars)?;
            Ok(CppType::Pointer(Box::new(head)))
        } else if input.contains("<") && input.contains(">") {
            // It must be a generic
            let re = generic_args_regex();
            let segments = &anyhow::Context::context(
                re.captures(input),
                "Cannot parse empty genric arguments",
            )?;
            let pre = &segments[1];

            let args: Vec<_> = split_into_arguments(&segments[2], '<', '>', ',');
            match pre.trim() {
                "std::vector" | "vector" => {
                    let ty = CppType::parse(args[0].trim())?;
                    Ok(CppType::Vec(Box::new(ty)))
                }
                "std::array" | "array" => {
                    let ty = CppType::parse(args[0].trim())?;
                    use std::str::FromStr;
                    let n = usize::from_str(args[1].trim())?;
                    Ok(CppType::Array(Box::new(ty), n))
                }
                "std::map" | "map" => {
                    let key = CppType::parse(args[0].trim())?;
                    let value = CppType::parse(args[1].trim())?;
                    Ok(CppType::Map(Box::new(key), Box::new(value)))
                }
                "std::list" | "list" => {
                    let ty = CppType::parse(args[0].trim())?;
                    Ok(CppType::LinkedList(Box::new(ty)))
                }
                "std::function" | "function" => {
                    let ty = segments[2].trim();
                    Ok(CppType::Function(Box::new(StdFunction::parse(ty)?)))
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
                        .map(|x| CppType::parse(&x))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(CppType::Generic { pre, args })
                }
            }
        } else if input.contains("::") {
            match input.trim() {
                "std::string" => Ok(CppType::String),
                "std::type_info" => Ok(CppType::TypeInfo),
                "std::size_t" => Ok(CppType::SizeT),
                // It must be some sort of path
                _ => Ok(CppType::Path(
                    input
                        .split("::")
                        .map(String::from)
                        .filter(|x| !x.is_empty())
                        .collect(),
                )),
            }
        } else {
            use CppType::*;
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
                "bool" => Ok(Bool),
                "FILE" => Ok(File),
                "FileMode" => Ok(FileMode),
                "string" => Ok(CppType::String),
                "type_info" => Ok(TypeInfo),
                "size_t" => Ok(SizeT),
                "char" => Ok(SignedChar),
                "ostream" => Ok(Ostream),
                other => {
                    if other.trim().contains(" ") {
                        anyhow::Context::context(None, "Paths must not contain spaces")
                    } else {
                        Ok(Path(vec![other.to_string()]))
                    }
                }
            }
        }
    }
}

impl CppType {
    fn parse_with_name(input: &str) -> Result<(Option<String>, Self)> {
        let arg = input.replace("&", " & ");
        let arg = arg.replace("*", " * ");

        let args = split_into_arguments(&arg, '<', '>', ' ');
        if args.is_empty() {
            use anyhow::Context;
            None.context("Cannot parse empty type declaration")?;
        }

        if let Ok(cpp_ty) = CppType::parse(&arg) {
            Ok((None, cpp_ty))
        } else {
            let name = args[args.len() - 1].clone();

            let n = args.len() - 1;
            let ty_input = args.into_iter().take(n).collect::<Vec<_>>().join(" ");

            let rtype = CppType::parse(&ty_input)?;

            Ok((Some(name), rtype))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StdFunction {
    return_type: CppType,
    args: Vec<CppType>,
}

impl Parse for StdFunction {
    fn parse(input: &str) -> Result<Self> {
        let re = regex::Regex::new(r#"([a-zA-Z0-9_: ]*)\((.*)\)"#).unwrap();

        let segments = &anyhow::Context::context(
            re.captures(input),
            format!("Cannot parse function signature: {}", input),
        )?;
        let outer = segments[1].trim();
        let inner = segments[2].trim();

        let args = split_into_arguments(inner, '<', '>', ',')
            .into_iter()
            .map(|arg| CppType::parse(&arg))
            .collect::<Result<Vec<_>>>()?;

        let return_type = CppType::parse(outer)?;

        Ok(Self { return_type, args })
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionSignature {
    return_type: CppType,
    name: String,
    args: Vec<(Option<String>, CppType)>,
}

#[derive(Debug, PartialEq)]
pub enum MemberKeyword {
    Static,
    Virtual,
}

impl Parse for MemberKeyword {
    fn parse(input: &str) -> Result<Self> {
        use MemberKeyword::*;
        use anyhow::Context;
        match input {
            "static" => Ok(Static),
            "virtual" => Ok(Virtual),
            _ => None.context(format!("cannot parse: {input} as MemberKeyword")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ClassMethod {
    keywords: Vec<MemberKeyword>,
    function_signature: FunctionSignature,
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
        let args = split_into_arguments(inner, '<', '>', ',')
            .into_iter()
            .map(|arg| CppType::parse_with_name(&arg))
            .collect::<Result<Vec<_>>>()?;

        let (name, return_type) = CppType::parse_with_name(outer)?;
        use anyhow::Context;
        let name = name.context("function declaration is missing name")?;

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
        let cpp_type = CppType::parse(t0)?;
        assert_eq!(cpp_type, CppType::SignedChar);

        let t1 = "unsigned char";
        let cpp_type = CppType::parse(t1)?;
        assert_eq!(cpp_type, CppType::UnsignedChar);

        Ok(())
    }

    #[test]
    fn parse_array() -> Result<()> {
        macro_rules! parse_array(
            ($arr:ident, $cppty:path, $n:literal) => {
                let parsed = CppType::parse($arr)?;
                if let CppType::Array(ty, n) = parsed {
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
        parse_array!(array1, CppType::Float, 3);
        let array2 = "std::array<double, 12>";
        parse_array!(array2, CppType::Double, 12);
        let array3 = "array<int, 4>";
        parse_array!(array3, CppType::Int, 4);

        Ok(())
    }

    #[test]
    fn parse_map() -> Result<()> {
        macro_rules! parse_map(
            ($map:ident, $cppkey:path, $cppvalue:path) => {
                let parsed = CppType::parse($map)?;
                if let CppType::Map(key, value) = parsed {
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
        parse_map!(map1, CppType::Int, CppType::Float);
        let map2 = "std::map<long, char>";
        parse_map!(map2, CppType::LongInt, CppType::SignedChar);
        let map3 = "map<unsigned char, double>";
        parse_map!(map3, CppType::UnsignedChar, CppType::Double);

        Ok(())
    }

    #[test]
    fn parse_list() -> Result<()> {
        macro_rules! parse_list(
            ($list:ident, $($cppty:tt)*) => {
                let parsed = CppType::parse($list)?;
                if let CppType::LinkedList(ty) = parsed {
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
        parse_list!(list1, CppType::Float);
        let list2 = "std::list<char>";
        parse_list!(list2, CppType::SignedChar);
        let list3 = "std::list<unsigned char>";
        parse_list!(list3, CppType::UnsignedChar);
        let list4 = "std::list<map<int, char>>";
        parse_list!(list4, CppType::Map(_, _));

        Ok(())
    }

    #[test]
    fn parse_vec() -> Result<()> {
        macro_rules! parse_vec(
            ($vec:ident, $($cppty:tt)*) => {
                let parsed = CppType::parse($vec)?;
                if let CppType::Vec(ty) = parsed {
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
        parse_vec!(vec1, CppType::LongInt);
        let vec2 = "std::vector<std::vector<int>>";
        parse_vec!(vec2, CppType::Vec(_));
        let vec3 = "vector<char>";
        parse_vec!(vec3, CppType::SignedChar);

        Ok(())
    }

    #[test]
    fn parse_path() -> Result<()> {
        macro_rules! parse_path(
            ($path:ident, $($segments:tt)*) => {
                let parsed = CppType::parse($path)?;
                if let CppType::Path(p) = parsed {
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
                if let CppType::Generic {
                    pre,
                    args,
                } = parsed {
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
        parse_generic!(generic1, "json", [CppType::Int, CppType::SignedChar]);
        let generic2 = "what::the<unsigned char, double, int>";
        parse_generic!(
            generic2,
            "what::the",
            [CppType::UnsignedChar, CppType::Double, CppType::Int]
        );

        Ok(())
    }

    #[test]
    fn parse_string() -> Result<()> {
        let cpp_type = CppType::parse("const std::string")?;
        assert_eq!(cpp_type, CppType::Const(Box::new(CppType::String)));

        Ok(())
    }

    #[test]
    fn parse_type_with_modifier() -> Result<()> {
        let cpp_type = CppType::parse("const int")?;
        assert_eq!(cpp_type, CppType::Const(Box::new(CppType::Int)));
        let cpp_type = CppType::parse("const int*")?;
        assert_eq!(
            cpp_type,
            CppType::Pointer(Box::new(CppType::Const(Box::new(CppType::Int))))
        );
        let cpp_type = CppType::parse("int* const")?;
        assert_eq!(
            cpp_type,
            CppType::Const(Box::new(CppType::Pointer(Box::new(CppType::Int))))
        );
        let cpp_type = CppType::parse("&float")?;
        assert_eq!(cpp_type, CppType::Ref(Box::new(CppType::Float)));
        let cpp_type = CppType::parse("char*")?;
        assert_eq!(cpp_type, CppType::Pointer(Box::new(CppType::SignedChar)));
        let cpp_type = CppType::parse("unsigned char")?;
        assert_eq!(cpp_type, CppType::UnsignedChar);

        Ok(())
    }

    #[test]
    fn parse_function_signature() -> Result<()> {
        let function_signature_1 = "void calc()";
        let signature1 = FunctionSignature::parse(function_signature_1)?;
        assert_eq!(signature1.name, "calc");
        assert_eq!(signature1.return_type, CppType::Void);
        assert!(signature1.args.is_empty());

        let function_signature_2 = "void xapy(float, float&, float*)";
        let signature2 = FunctionSignature::parse(function_signature_2)?;
        assert_eq!(signature2.args.len(), 3);
        assert_eq!(signature2.args[0].1, CppType::Float);
        assert_eq!(signature2.args[1].1, CppType::Ref(Box::new(CppType::Float)));
        assert_eq!(
            signature2.args[2].1,
            CppType::Pointer(Box::new(CppType::Float))
        );

        let function_signature_2 = "void IsType(const char *type)";
        let signature2 = FunctionSignature::parse(function_signature_2)?;
        assert_eq!(signature2.args.len(), 1);
        assert_eq!(
            signature2.args[0].1,
            CppType::Pointer(Box::new(CppType::Const(Box::new(CppType::SignedChar))))
        );

        Ok(())
    }

    #[test]
    fn parse_std_function() -> Result<()> {
        let std_function_ty = "std::function<bool(float, float)>";
        let ty = CppType::parse(std_function_ty)?;
        assert_eq!(
            ty,
            CppType::Function(Box::new(StdFunction {
                return_type: CppType::Bool,
                args: vec![CppType::Float, CppType::Float]
            }))
        );
        Ok(())
    }
}
