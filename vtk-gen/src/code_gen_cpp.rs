use crate::Result;
use crate::intermediate_representation::{IRMethod, IRModule, IRStruct, IRType};

macro_rules! _cpp(
    (#(#$to:ident,)*) => {{
        let mut out = String::new();
        for _item in $to {
            if out.is_empty() {
                out = format!("{}", cpp!(#_item)?.trim());
            } else {
                out = format!("{out}, {}", cpp!(#_item)?.trim());
            }
        }
        out
    }};
    (#$id:ident $($tk:tt)*) => {format!("{} {}", ($id).to_cpp_str()?.as_ref(), _cpp!($($tk)*))};
    (#$id:ident) => {format!("{}", ($id).to_cpp_str()?.as_ref())};
    ({$($gr:tt)+}) => {format!("{{{}}}", _cpp!($($gr)+))};
    ({$($gr:tt)+} $($tk:tt)*) => {format!("{{{}}} {}", _cpp!($($gr)+), _cpp!($($tk)*))};
    ($tk:tt) => {format!("{}", stringify!($tk))};
    (($($gr:tt)+) $($tk:tt)*) => {format!("({}) {}", _cpp!($($gr)+), _cpp!($($tk)*))};
    ($tk1:tt $($tk:tt)*) => {format!("{} {}", _cpp!($tk1), _cpp!($($tk)*))};
    () => {"".to_string()};
);

macro_rules! cpp(
    ($($tk:tt)*) => {{
        let __internal_doer = || -> crate::Result<String> {
            let out: String = _cpp!($($tk)*);
            Ok(out)
        };
        __internal_doer()
    }}
);

pub trait FormatCppStr {
    fn to_cpp_str(&self) -> Result<impl AsRef<str>>;
}

pub trait FormatCpp {
    fn to_cpp(&self, writer: &mut impl std::io::Write) -> Result<()>;
}

impl<T> FormatCpp for T
where
    T: FormatCppStr,
{
    fn to_cpp(&self, writer: &mut impl std::io::Write) -> Result<()> {
        write!(writer, "{}", self.to_cpp_str()?.as_ref())?;
        Ok(())
    }
}

impl FormatCppStr for String {
    fn to_cpp_str(&self) -> Result<impl AsRef<str>> {
        Ok(self.as_str())
    }
}

impl FormatCppStr for &str {
    fn to_cpp_str(&self) -> Result<impl AsRef<str>> {
        Ok(self)
    }
}

impl FormatCppStr for IRType {
    fn to_cpp_str(&self) -> Result<impl AsRef<str>> {
        use IRType::*;
        match self {
            Unit => Ok("void"),
            c_uchar => Ok("unsigned char"),
            c_ushort => Ok("unsigned short"),
            c_uint => Ok("unsigned int"),
            c_ulong => Ok("unsigned long"),
            c_ulonglong => Ok("unsigned long long"),
            c_char => Ok("char"),
            c_signed_char => Ok("signed char"),
            c_short => Ok("short"),
            c_int => Ok("int"),
            c_long => Ok("long"),
            c_longlong => Ok("long long"),
            bool => Ok("bool"),
            float => Ok("float"),
            double => Ok("double"),
            usize => Ok("size_t"),

            _ => anyhow::bail!("type not implemented yet"),
        }
    }
}

fn is_numeric_primitive(ty: &IRType) -> bool {
    use IRType::*;
    // c_char is excluded: char* is a C string, not an array.
    // c_signed_char IS included: signed char* is a typed data buffer (e.g. vtkSignedCharArray),
    // and char* ↔ signed char* is an error in C++ so we must not emit a pointer to it.
    matches!(
        ty,
        c_signed_char
            | c_uchar
            | c_short
            | c_ushort
            | c_int
            | c_uint
            | c_long
            | c_ulong
            | c_longlong
            | c_ulonglong
            | float
            | double
            | usize
            | bool
    )
}

fn ir_type_to_cpp_string(irtype: &IRType) -> Result<String> {
    use IRType::*;
    match irtype {
        Unit => Ok("void".to_string()),
        c_uchar => Ok("unsigned char".to_string()),
        c_ushort => Ok("unsigned short".to_string()),
        c_uint => Ok("unsigned int".to_string()),
        c_ulong => Ok("unsigned long".to_string()),
        c_ulonglong => Ok("unsigned long long".to_string()),
        c_char => Ok("char".to_string()),
        c_signed_char => Ok("signed char".to_string()),
        c_short => Ok("short".to_string()),
        c_int => Ok("int".to_string()),
        c_long => Ok("long".to_string()),
        c_longlong => Ok("long long".to_string()),
        bool => Ok("bool".to_string()),
        float => Ok("float".to_string()),
        double => Ok("double".to_string()),
        usize => Ok("size_t".to_string()),
        String => Ok("const char*".to_string()),
        Const(inner) => match inner.as_ref() {
            String => Ok("const char*".to_string()),
            other => Ok(format!("const {}", ir_type_to_cpp_string(other)?)),
        },
        Pointer(inner) => {
            // Strip one layer of Const to check the core element type
            let core = match inner.as_ref() {
                Const(x) => x.as_ref(),
                x => x,
            };
            if is_numeric_primitive(core) {
                anyhow::bail!("pointer-to-numeric-primitive (array arg) not bridgeable")
            }
            // Mutable char* is not bridgeable: VTK may expect signed char* (data arrays),
            // and C++ rejects implicit conversion between char* and signed char*.
            if matches!(inner.as_ref(), c_char) {
                anyhow::bail!("mutable char* not bridgeable (may conflict with signed char*)")
            }
            Ok(format!("{}*", ir_type_to_cpp_string(inner)?))
        }
        Ref(inner) => Ok(format!("{}&", ir_type_to_cpp_string(inner)?)),
        _ => anyhow::bail!("C++ type not supported: skipping method"),
    }
}

impl IRModule {
    fn write_includes(&self, writer: &mut impl std::io::Write) -> Result<()> {
        writeln!(writer, "// Default include in all modules")?;
        writeln!(writer, "#include<vtkNew.h>")?;
        writeln!(writer, "#include<vtkObjectBase.h>")?;
        writeln!(writer)?;

        writeln!(writer, "// Include objects of this module")?;
        for (_, ir_struct) in self.classes.iter() {
            writeln!(writer, "#include<{}>", ir_struct.filename)?;
        }
        Ok(())
    }

    pub(crate) fn to_cpp_src(&self, writer: &mut impl std::io::Write) -> Result<()> {
        writeln!(writer, "// Include header file")?;
        writeln!(writer, "#include<{}.h>", self.name_snake_case())?;
        writeln!(writer)?;
        self.write_includes(writer)?;
        writeln!(writer)?;
        writeln!(writer, "// Implement declared functions")?;

        for (_, irstruct) in self.classes.iter() {
            if irstruct.is_constructable() {
                irstruct.build_constructor(writer)?;
                for method in irstruct.exposable_methods.iter() {
                    if let Err(e) = irstruct.method_to_cpp(method, writer) {
                        log::warn!(
                        "[Cpp] skipping method \"{}\" due to error: \"{e}\"",
                        method.name
                    );
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn to_cpp_header(&self, writer: &mut impl std::io::Write) -> Result<()> {
        self.write_includes(writer)?;
        writeln!(writer)?;

        writeln!(writer, "// Declare exported functions")?;
        for (_, irstruct) in self.classes.iter() {
            if irstruct.is_constructable() {
                irstruct.build_constructor_headers(writer)?;
                for method in irstruct.exposable_methods.iter() {
                    if let Err(e) = irstruct.method_to_cpp_header(method, writer) {
                        log::warn!(
                            "[Cpp] skipping method header \"{}\" due to: \"{e}\"",
                            method.name
                        );
                    }
                }
            }
        }
        Ok(())
    }
}

impl IRModule {
    pub(crate) fn name_snake_case(&self) -> String {
        convert_case::ccase!(snake, &self.name)
    }

    pub(crate) fn vtk_module_name(&self) -> Option<String> {
        // TODO this is probably incorrect and needs to be addressed somehow differently
        self.name.split("vtk").nth(1).map(|x| x.to_string())
    }
}

impl IRStruct {
    fn method_to_cpp(&self, method: &IRMethod, writer: &mut impl std::io::Write) -> Result<()> {
        if matches!(method.return_type, IRType::String) {
            anyhow::bail!("std::string return type cannot be bridged to const char*");
        }
        let ret_str = ir_type_to_cpp_string(&method.return_type)?;
        let is_void = matches!(method.return_type, IRType::Unit);

        let mut param_strs: Vec<String> = vec![format!("{}* sself", self.name)];
        let mut call_args: Vec<String> = vec![];
        for (ident, irtype) in &method.args {
            param_strs.push(format!("{} {}", ir_type_to_cpp_string(irtype)?, ident.0));
            call_args.push(ident.0.clone());
        }

        let params = param_strs.join(", ");
        let args = call_args.join(", ");
        let body = if is_void {
            format!("sself->{}({});", method.vtk_name, args)
        } else {
            format!("return sself->{}({});", method.vtk_name, args)
        };

        writeln!(
            writer,
            "extern \"C\" {} {}({}) {{ {} }}",
            ret_str, method.name, params, body
        )?;
        Ok(())
    }

    fn method_to_cpp_header(
        &self,
        method: &IRMethod,
        writer: &mut impl std::io::Write,
    ) -> Result<()> {
        if matches!(method.return_type, IRType::String) {
            anyhow::bail!("std::string return type cannot be bridged to const char*");
        }
        let ret_str = ir_type_to_cpp_string(&method.return_type)?;
        let mut param_strs: Vec<String> = vec![format!("{}* sself", self.name)];
        for (ident, irtype) in &method.args {
            param_strs.push(format!("{} {}", ir_type_to_cpp_string(irtype)?, ident.0));
        }
        let params = param_strs.join(", ");
        writeln!(
            writer,
            "extern \"C\" {} {}({});",
            ret_str, method.name, params
        )?;
        Ok(())
    }

    fn build_constructor(&self, writer: &mut impl std::io::Write) -> Result<()> {
        let ty = &self.name;
        let constructor = self.constructor_binding_name();
        let func1 = cpp!(extern "C" #ty* #constructor() {return #ty::New();})?;

        let destructor = self.destructor_binding_name();
        let func2 = cpp!(extern "C" void #destructor(#ty* sself) {
            sself->Delete();
            return;
        })?;

        writeln!(writer, "{func1}")?;
        writeln!(writer, "{func2}")?;
        Ok(())
    }

    fn build_constructor_headers(&self, writer: &mut impl std::io::Write) -> Result<()> {
        let ty = &self.name;
        let constructor = self.constructor_binding_name();
        let func1 = cpp!(extern "C" #ty* #constructor();)?;

        let destructor = self.destructor_binding_name();
        let func2 = cpp!(extern "C" void #destructor(#ty* sself);)?;

        writeln!(writer, "{func1}")?;
        writeln!(writer, "{func2}")?;
        Ok(())
    }
}

/* impl FormatCpp for Option<ReturnType> {
    fn to_cpp(&self, writer: &mut impl core::fmt::Write) -> Result<()> {
        match self {
            Some(r) => {
                if let Some(p) = &r.pointer {
                    match p {
                        Pointer::Ref => return Err(anyhow::format_err!("cannot return reference")),
                        Pointer::Star => write!(writer, "void*")?,
                        Pointer::StarStar => {
                            return Err(anyhow::format_err!("cannot return double-starred type"));
                        }
                    }
                } else {
                    write!(writer, "{}", r.ret_type)?;
                }
            }
            None => write!(writer, "void")?,
        }
        Ok(())
    }
}

impl FormatCpp for CppType {
    fn to_cpp(&self, writer: &mut impl core::fmt::Write) -> Result<()> {
        /* let CppType {
            name,
            r#type,
            reference,
        } = self;*/

        /* write!(writer, "{}", r#type)?;
        if *reference {
            // Format reference to pointer
            write!(writer, "* ")?;
        } else {
            write!(writer, " ")?;
        }
        if let Some(name) = name {
            write!(writer, "{}", name)?;
        }*/

        Ok(())
    }
}

impl FormatCpp for IRMethod {
    fn to_cpp(&self, writer: &mut impl core::fmt::Write) -> Result<()> {
        let IRMethod {
            name,
            return_type,
            args,
        } = self;

        /* if let Some(comment) = comment {
            for line in comment.lines() {
                writeln!(writer, "// {}", line.trim())?;
            }
        }
        return_type.to_cpp(writer)?;
        // TODO use correct type of class
        write!(writer, " {}(void* classPtr", name)?;

        for p in parameters.iter() {
            write!(writer, ", ")?;

            // TODO use generated name if non is provided
            p.to_cpp(writer)?;
        }

        writeln!(writer, ") {{")?;
        write!(
            writer,
            "    return reinterpret_cast<vtkNew<vtkObject>>(classPtr)->{}(",
            name
        )?;

        for (i, p) in parameters.iter().enumerate() {
            write!(writer, "{}", p.name.clone().unwrap_or_default())?;
            if i + 1 < parameters.len() {
                write!(writer, ", ")?;
            }
        }
        writeln!(writer, ");")?;
        writeln!(writer, "}}")?;*/
        Ok(())
    }
}*/

#[cfg(test)]
mod gen_cpp_tests {
    use super::*;
    use crate::intermediate_representation::{IRIdent, IRMethod, IRStruct};

    #[test]
    fn test_cpp_macro() {
        let out = cpp!(extern "C" void use_this(void* ptr) {return;}).unwrap();
        assert_eq!(out, "extern \"C\" void use_this (void * ptr) {return ;}");
    }

    #[test]
    fn test_cpp_macro_repetition() {
        let args = vec!["int indent", "char* stream", "bool flag"];
        let out = cpp!(void do_stuff(#(#args,)*) { return; }).unwrap();
        assert_eq!(
            out,
            "void do_stuff (int indent, char* stream, bool flag) {return ;}"
        );
    }

    #[test]
    fn test_cpp_type_primitives() {
        assert_eq!(ir_type_to_cpp_string(&IRType::Unit).unwrap(), "void");
        assert_eq!(ir_type_to_cpp_string(&IRType::c_int).unwrap(), "int");
        assert_eq!(ir_type_to_cpp_string(&IRType::double).unwrap(), "double");
        assert_eq!(ir_type_to_cpp_string(&IRType::bool).unwrap(), "bool");
        assert_eq!(ir_type_to_cpp_string(&IRType::float).unwrap(), "float");
        assert_eq!(ir_type_to_cpp_string(&IRType::usize).unwrap(), "size_t");
    }

    #[test]
    fn test_cpp_type_string_maps_to_const_char_ptr() {
        assert_eq!(
            ir_type_to_cpp_string(&IRType::String).unwrap(),
            "const char*"
        );
    }

    #[test]
    fn test_cpp_type_const_string_maps_to_const_char_ptr() {
        assert_eq!(
            ir_type_to_cpp_string(&IRType::Const(Box::new(IRType::String))).unwrap(),
            "const char*"
        );
    }

    #[test]
    fn test_cpp_type_pointer_wraps_inner() {
        assert_eq!(
            ir_type_to_cpp_string(&IRType::Pointer(Box::new(IRType::Unit))).unwrap(),
            "void*"
        );
    }

    #[test]
    fn test_cpp_type_unsupported_returns_error() {
        assert!(ir_type_to_cpp_string(&IRType::FileMode).is_err());
    }

    fn make_struct(name: &str) -> IRStruct {
        IRStruct::test_new(name, vec!["vtkObjectBase"], vec![])
    }

    #[test]
    fn test_method_to_cpp_void_no_args() {
        let s = make_struct("vtkFoo");
        let m = IRMethod::test_new("vtk_foo_update", "Update", IRType::Unit, vec![]);
        let mut out = Vec::new();
        s.method_to_cpp(&m, &mut out).unwrap();
        let src = String::from_utf8(out).unwrap();
        assert!(src.contains("extern \"C\" void vtk_foo_update"));
        assert!(src.contains("vtkFoo* sself"));
        assert!(src.contains("sself->Update()"));
    }

    #[test]
    fn test_method_to_cpp_with_args_and_return() {
        let s = make_struct("vtkFoo");
        let m = IRMethod::test_new(
            "vtk_foo_set_radius",
            "SetRadius",
            IRType::double,
            vec![(IRIdent("r".to_string()), IRType::double)],
        );
        let mut out = Vec::new();
        s.method_to_cpp(&m, &mut out).unwrap();
        let src = String::from_utf8(out).unwrap();
        assert!(src.contains("extern \"C\" double vtk_foo_set_radius"));
        assert!(src.contains("double r"));
        assert!(src.contains("return sself->SetRadius(r)"));
    }

    #[test]
    fn test_method_to_cpp_header_declaration() {
        let s = make_struct("vtkFoo");
        let m = IRMethod::test_new("vtk_foo_update", "Update", IRType::Unit, vec![]);
        let mut out = Vec::new();
        s.method_to_cpp_header(&m, &mut out).unwrap();
        let src = String::from_utf8(out).unwrap();
        assert!(src.contains("extern \"C\" void vtk_foo_update"));
        assert!(src.ends_with(";\n"));
        assert!(!src.contains("sself->"), "header must not contain body");
    }
}