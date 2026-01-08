use crate::Result;
use crate::intermediate_representation::{IRMethod, IRModule, IRType};

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

impl FormatCpp for IRModule {
    fn to_cpp(&self, writer: &mut impl std::io::Write) -> Result<()> {
        for (_, ir_struct) in self.classes.iter() {
            for method in ir_struct.exposable_methods.iter() {
                match method.to_cpp(writer) {
                    Ok(_) => (),
                    Err(e) => log::warn!(
                        "[Cpp] skipping method \"{}\" due to error: \"{e}\"",
                        method.name
                    ),
                }
            }
        }
        Ok(())
    }
}

impl FormatCpp for IRMethod {
    fn to_cpp(&self, writer: &mut impl std::io::Write) -> Result<()> {
        writeln!(
            writer,
            "{} {}() {{",
            self.return_type.to_cpp_str()?.as_ref(),
            self.name
        )?;
        writeln!(writer, "}}")?;
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
