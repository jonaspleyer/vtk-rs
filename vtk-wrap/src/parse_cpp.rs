use crate::Result;

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
    Array(Box<CppType>, usize),
    Vec(Box<CppType>),
    Map(Box<CppType>, Box<CppType>),
    LinkedList(Box<CppType>),
    Generic {
        pre: Box<CppType>,
        args: Vec<Box<CppType>>,
    },
    Path(syn::Path),
}

fn generic_args_regex() -> regex::Regex {
    regex::Regex::new(r#"([a-zA-Z0-9:]*)<(.*)>"#).unwrap()
}

impl CppType {
    fn parse(input: &str) -> Result<Self> {
        if input.contains("<") && input.contains(">") {
            // It must be a generic
            let re = generic_args_regex();
            let segments = &re.captures(input).unwrap();
            let pre = &segments[1];
            let args: Vec<_> = segments[2].split(",").collect();
            match pre.trim() {
                "std::array" | "array" => {
                    let ty = CppType::parse(args[0])?;
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
                _ => todo!(),
            }
        } else if input.contains("::") {
            // It must be some sort of path
            let path: syn::Path = syn::parse_str(input)?;
            Ok(CppType::Path(path))
        } else {
            use CppType::*;
            match input.trim() {
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
                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        let array3 = "std::array<int, 4>";
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
        let map3 = "std::map<unsigned char, double>";
        parse_map!(map3, CppType::UnsignedChar, CppType::Double);

        Ok(())
    }

    #[test]
    fn parse_list() -> Result<()> {
        macro_rules! parse_list(
            ($list:ident, $cppty:path) => {
                let parsed = CppType::parse($list)?;
                if let CppType::LinkedList(ty) = parsed {
                    match ty.as_ref() {
                        $cppty => (),
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

        Ok(())
    }
}
