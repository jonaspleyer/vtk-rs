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
        println!("{input}");
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
    fn parse_hashmap() -> Result<()> {
        // let hashmap1 = "std::array"
        Ok(())
    }
}
