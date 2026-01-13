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
    pub(crate) fn to_cpp_src(&self, writer: &mut impl std::io::Write) -> Result<()> {
        // Include vtk libraries required
        writeln!(writer, "#include<vtkNew>;")?;
        writeln!(writer, "#include<{}.h>;", self.name)?;
        writeln!(writer)?;

        // Include other cpp libraries required

        // Include header file
        writeln!(writer, "#include<{}.h>;", self.name_snake_case())?;
        writeln!(writer)?;

        for (_, ir_struct) in self.classes.iter() {
            ir_struct.build_constructor(writer)?;
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
        for (_, ir_struct) in self.classes.iter() {
            ir_struct.build_constructor_headers(writer)?;
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
        self.name.split("vtk").next().map(|x| x.to_string())
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

        let spointer = if params.is_empty() {
            "void* self"
        } else {
            "void* self, "
        };
        writeln!(
            writer,
            "{} {}({spointer}{params}) {{",
            method.return_type.to_cpp_str()?.as_ref(),
            method.name
        )?;
        writeln!(
            writer,
            "    return dynamic_cast<{}*>(self)->MethodName(args);",
            self.name
        )?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    fn build_constructor(&self, writer: &mut impl std::io::Write) -> Result<()> {
        let ty = &self.name;
        let constructor = self.constructor_name();
        writeln!(writer, "void* {}() {{", constructor)?;
        writeln!(writer, "    return vtkNew<{ty}>();")?;
        writeln!(writer, "}}")?;

        /* let copy_constructor = self.copy_constructor();
        writeln!(writer)?;
        writeln!(writer, "void* {}(void* sself) {{", copy_constructor)?;
        writeln!(
            writer,
            "    {ty}* ptr = dynamic_cast<vtkNew<{ty}>>(sself)->GetPointer();"
        )?;
        writeln!(writer, "    {ty}")
        writeln!(writer, "    return vtkNew<{ty}>({ptr}(sself)", self.name)?;
        writeln!(writer, "}}")?;*/
        Ok(())
    }

    fn build_constructor_headers(&self, writer: &mut impl std::io::Write) -> Result<()> {
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
