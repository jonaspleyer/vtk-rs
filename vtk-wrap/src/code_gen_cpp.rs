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

impl IRModule {
    fn write_includes(&self, writer: &mut impl std::io::Write) -> Result<()> {
        writeln!(writer, "// Default include in all modules")?;
        writeln!(writer, "#include<vtkNew.h>")?;
        writeln!(writer, "#include<vtkObjectBase.h>")?;
        writeln!(writer)?;

        writeln!(writer, "// Include objects of this module")?;
        for (_, ir_struct) in self.classes.iter() {
            // if ir_struct.is_constructable() {
            writeln!(writer, "#include<{}>", ir_struct.filename)?;
            // }
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

        // Include vtk libraries required
        for (_, ir_struct) in self.classes.iter() {
            if ir_struct.is_constructable() {
                ir_struct.build_constructor(writer)?;
            }
            /* for method in ir_struct.exposable_methods.iter() {
                match ir_struct.method_to_cpp(method, writer) {
                    Ok(_) => (),
                    Err(e) => log::warn!(
                        "[Cpp] skipping method \"{}\" due to error: \"{e}\"",
                        method.name
                    ),
                }
            }*/
        }
        Ok(())
    }

    pub(crate) fn to_cpp_header(&self, writer: &mut impl std::io::Write) -> Result<()> {
        self.write_includes(writer)?;
        writeln!(writer)?;

        writeln!(writer, "// Declare exported functions")?;
        for (_, ir_struct) in self.classes.iter() {
            if ir_struct.is_constructable() {
                ir_struct.build_constructor_headers(writer)?;
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
        let mut params = String::new();
        for (n, (ident, ty)) in method.args.iter().enumerate() {
            params.push_str(ty.to_cpp_str()?.as_ref());
            params.push(' ');
            params.push_str(&ident.0);
            if n + 1 < method.args.len() {
                params.push_str(", ");
            }
        }

        let ty = &self.name;
        let spointer = if params.is_empty() {
            cpp!(vtkNew<#ty> self)?
        } else {
            cpp!(vtkNew<#ty> self)?
        };

        let ret = &method.return_type;
        let vtk_name = &method.name;
        let binding = format!("{}_{}", self.name, method.name);
        let method = cpp!(#ret #binding (#spointer, #(#params),*) {
            return self->#vtk_name(#(params),*);
        })?;
        writeln!(writer, "{}", method)?;

        Ok(())
    }

    fn build_constructor(&self, writer: &mut impl std::io::Write) -> Result<()> {
        let ty = &self.name;
        let constructor = self.constructor_binding_name();
        let func1 = cpp!(extern "C" vtkNew<#ty> #constructor() {return vtkNew<#ty>();})?;

        let destructor = self.destructor_binding_name();
        let func2 = cpp!(extern "C" void #destructor(vtkNew<#ty> sself) {
            sself.Reset();
            return;
        })?;

        let get_ptr = self.get_ptr_binding_name();
        let func3 = cpp!(extern "C" void* #get_ptr(vtkNew<#ty> sself) {
            return sself.GetPointer();
        })?;

        writeln!(writer, "{func1}")?;
        writeln!(writer, "{func2}")?;
        writeln!(writer, "{func3}")?;
        Ok(())
    }

    fn build_constructor_headers(&self, writer: &mut impl std::io::Write) -> Result<()> {
        let ty = &self.name;
        let constructor = self.constructor_binding_name();
        let func1 = cpp!(extern "C" vtkNew<#ty> #constructor();)?;

        let destructor = self.destructor_binding_name();
        let func2 = cpp!(extern "C" void #destructor(vtkNew<#ty> sself);)?;

        let get_ptr = self.get_ptr_binding_name();
        let func3 = cpp!(extern "C" void* #get_ptr(vtkNew<#ty> sself);)?;

        writeln!(writer, "{func1}")?;
        writeln!(writer, "{func2}")?;
        writeln!(writer, "{func3}")?;
        Ok(())
    }
}

#[test]
fn test_cpp_macro() {
    let out = cpp!(extern "C" void use_this(void* ptr) {return;}).unwrap();
    assert_eq!(out, "extern \"C\" void use_this (void * ptr) {return ;}");
}

#[test]
fn test_cpp_macro_repitition() {
    let args = vec!["int indent", "char* stream", "bool flag"];
    let out = cpp!(void do_stuff(#(#args,)*) { return; }).unwrap();
    assert_eq!(
        out,
        "void do_stuff (int indent, char* stream, bool flag) {return ;}"
    );
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
